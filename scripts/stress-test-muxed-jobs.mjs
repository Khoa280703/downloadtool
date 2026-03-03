import fs from 'node:fs/promises';

const API = process.env.API_BASE || 'http://127.0.0.1:3068';
const PAYLOAD_FILE = process.env.PAYLOAD_FILE;
const TOTAL = Number(process.env.CREATE_TOTAL || 1200);
const CREATE_CONC = Number(process.env.CREATE_CONC || 24);
const POLL_CONC = Number(process.env.POLL_CONC || 96);
const POLL_TIMEOUT_MS = Number(process.env.POLL_TIMEOUT_MS || 300000);
const POLL_INTERVAL_MS = Number(process.env.POLL_INTERVAL_MS || 1000);

if (!PAYLOAD_FILE) {
  console.error('PAYLOAD_FILE is required');
  process.exit(1);
}

function percentile(values, p) {
  if (!values.length) return 0;
  const sorted = [...values].sort((a, b) => a - b);
  return sorted[Math.floor((sorted.length - 1) * p)] || 0;
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function pool(total, conc, worker) {
  let idx = 0;
  const out = new Array(total);
  async function run() {
    while (true) {
      const current = idx++;
      if (current >= total) return;
      out[current] = await worker(current);
    }
  }
  await Promise.all(Array.from({ length: conc }, run));
  return out;
}

const payload = JSON.parse(await fs.readFile(PAYLOAD_FILE, 'utf8'));

const creates = await pool(TOTAL, CREATE_CONC, async () => {
  const t0 = Date.now();
  try {
    const response = await fetch(`${API}/api/stream/muxed/jobs`, {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify(payload),
    });
    const text = await response.text();
    let parsed = null;
    try {
      parsed = text ? JSON.parse(text) : null;
    } catch {
      parsed = null;
    }
    return {
      code: response.status,
      ms: Date.now() - t0,
      jobId: parsed?.job_id || '',
      startedAt: t0,
    };
  } catch {
    return { code: 0, ms: Date.now() - t0, jobId: '', startedAt: t0 };
  }
});

const accepted = creates.filter((x) => x.code === 202 && x.jobId);
async function pollAcceptedJobs() {
  if (!accepted.length) return { polls: [], httpStatusCounts: {} };

  const states = accepted.map((item) => ({
    jobId: item.jobId,
    startAt: item.startedAt || Date.now(),
    done: false,
    final: '',
    ms: 0,
  }));
  const httpStatusCounts = {};

  async function pollOne(state) {
    try {
      const response = await fetch(`${API}/api/stream/muxed/jobs/${encodeURIComponent(state.jobId)}`);
      const statusCode = String(response.status);
      httpStatusCounts[statusCode] = (httpStatusCounts[statusCode] || 0) + 1;
      if (response.status === 200) {
        const body = await response.json();
        const status = body?.status || '';
        if (status === 'ready' || status === 'failed') {
          state.done = true;
          state.final = status;
          state.ms = Date.now() - state.startAt;
        }
      } else if (response.status === 404) {
        state.done = true;
        state.final = 'not_found';
        state.ms = Date.now() - state.startAt;
      }
    } catch {
      // Ignore transient errors and keep polling.
    }
  }

  while (true) {
    const pending = states.filter((x) => !x.done);
    if (!pending.length) break;

    const now = Date.now();
    for (const state of pending) {
      if (now - state.startAt >= POLL_TIMEOUT_MS) {
        state.done = true;
        state.final = 'timeout';
        state.ms = now - state.startAt;
      }
    }

    const stillPending = states.filter((x) => !x.done);
    if (!stillPending.length) break;

    for (let i = 0; i < stillPending.length; i += POLL_CONC) {
      const chunk = stillPending.slice(i, i + POLL_CONC);
      await Promise.all(chunk.map((x) => pollOne(x)));
    }

    const afterRoundPending = states.filter((x) => !x.done);
    if (!afterRoundPending.length) break;
    await sleep(POLL_INTERVAL_MS);
  }

  return {
    polls: states.map((x) => ({ final: x.final, ms: x.ms })),
    httpStatusCounts,
  };
}

const pollResult = await pollAcceptedJobs();
const polls = pollResult.polls;

const codeCounts = {};
for (const create of creates) {
  const key = String(create.code);
  codeCounts[key] = (codeCounts[key] || 0) + 1;
}

const finalCounts = {};
for (const poll of polls) {
  const key = poll.final;
  finalCounts[key] = (finalCounts[key] || 0) + 1;
}
const readyWithin5m = polls.filter((x) => x.final === 'ready' && x.ms <= 300000).length;
const eventualReady = finalCounts.ready || 0;
const acceptedTotal = accepted.length;

const out = {
  apiBase: API,
  create: {
    total: TOTAL,
    concurrency: CREATE_CONC,
    codeCounts,
    accepted: accepted.length,
    p50Ms: percentile(
      creates.map((x) => x.ms),
      0.5,
    ),
    p95Ms: percentile(
      creates.map((x) => x.ms),
      0.95,
    ),
    p99Ms: percentile(
      creates.map((x) => x.ms),
      0.99,
    ),
  },
  poll: {
    total: polls.length,
    concurrency: POLL_CONC,
    finalCounts,
    httpStatusCounts: pollResult.httpStatusCounts,
    readyWithin5m,
    eventualReady,
    readyWithin5mPctOfAccepted:
      acceptedTotal > 0 ? Number(((readyWithin5m / acceptedTotal) * 100).toFixed(2)) : 0,
    eventualReadyPctOfAccepted:
      acceptedTotal > 0 ? Number(((eventualReady / acceptedTotal) * 100).toFixed(2)) : 0,
    p50Ms: percentile(
      polls.map((x) => x.ms),
      0.5,
    ),
    p95Ms: percentile(
      polls.map((x) => x.ms),
      0.95,
    ),
    p99Ms: percentile(
      polls.map((x) => x.ms),
      0.99,
    ),
  },
};

console.log(JSON.stringify(out, null, 2));
