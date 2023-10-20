CREATE TABLE AppLogSettings(
    app_id UUID NOT NULL,
    retention_days INTEGER DEFAULT 30 NOT NULL,
    daily_export BOOLEAN DEFAULT TRUE NOT NULL,
    ignore_debug BOOLEAN DEFAULT TRUE NOT NULL,
    created_date TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_date TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL
);
ALTER TABLE AppLogSettings ADD PRIMARY KEY(app_id);
CREATE TABLE Application(
    id UUID DEFAULT gen_random_uuid() NOT NULL,
    name VARCHAR(255) NOT NULL,
    key VARCHAR(255) NOT NULL,
    created_date TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL
);
ALTER TABLE Application ADD PRIMARY KEY(id);
ALTER TABLE Application ADD CONSTRAINT application_key_unique UNIQUE(key);
CREATE INDEX application_name_index ON Application(name);
CREATE INDEX application_key_index ON Application(key);
CREATE TYPE LogLevel AS ENUM ('DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL', 'CRASH');
CREATE TABLE Log(
    app_id UUID NOT NULL,
    loged_at TIMESTAMP WITH TIME zone DEFAULT now() NOT NULL,
    log_level LogLevel NOT NULL,
    log_message TEXT NOT NULL
);
ALTER TABLE Log ADD PRIMARY KEY(app_id, loged_at);
ALTER TABLE Log ADD CONSTRAINT log_app_id_foreign FOREIGN KEY(app_id) REFERENCES Application(id);
ALTER TABLE AppLogSettings ADD CONSTRAINT applogsettings_app_id_foreign FOREIGN KEY(app_id) REFERENCES Application(id);
