-- Create the llm_models table
CREATE TABLE IF NOT EXISTS llm_models (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    model_name TEXT NOT NULL UNIQUE,
    provider TEXT NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT 0
);

-- Insert default OpenAI models
INSERT INTO llm_models (model_name, provider, is_default) VALUES
('gpt-4o', 'openai', 1),
('gpt-4o-mini', 'openai', 0),
('gpt-4-turbo', 'openai', 0),
('gpt-3.5-turbo', 'openai', 0);