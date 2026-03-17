ALTER TABLE mux_jobs
    ADD COLUMN IF NOT EXISTS preferred_video_proxy TEXT,
    ADD COLUMN IF NOT EXISTS preferred_audio_proxy TEXT;
