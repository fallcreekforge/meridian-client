-- Declarative SQLite schema.
-- Define tables, indexes, and constraints here.

CREATE TABLE IF NOT EXISTS mer_platform (
    platform_id INTEGER PRIMARY KEY,
    name        TEXT
);

CREATE TABLE IF NOT EXISTS mer_error (
    error_id    INTEGER PRIMARY KEY,
    message     TEXT,
    stack_trace TEXT
);

-- Single Table Inheritance table for configured applications.
-- Will be expanded with more columns for each platform.
CREATE TABLE IF NOT EXISTS mer_application (
    application_id          INTEGER PRIMARY KEY,
    steam_app_id            INTEGER UNIQUE,
    steam_app_type_id       INTEGER,
    steam_app_name          TEXT,
    -- ... Add platforms
    meridian_application_id TEXT,

    FOREIGN KEY (steam_app_type_id) REFERENCES steam_app_type (
        steam_app_type_id
    )
);

CREATE TABLE IF NOT EXISTS mer_batch_process (
    batch_process_id        INTEGER PRIMARY KEY,
    process_start           TEXT,
    process_end             TEXT,
    batch_process_type_id   INTEGER,
    batch_process_status_id INTEGER,
    platform_id             INTEGER,

    FOREIGN KEY (batch_process_type_id) REFERENCES mer_batch_process_type (
        batch_process_type_id
    ),
    FOREIGN KEY (batch_process_status_id) REFERENCES mer_batch_process_status (
        batch_process_status_id
    ),
    FOREIGN KEY (platform_id) REFERENCES mer_platform (platform_id)
);

CREATE TABLE IF NOT EXISTS mer_batch_process_type (
    batch_process_type_id INTEGER PRIMARY KEY,
    name                  TEXT
);

CREATE TABLE IF NOT EXISTS mer_batch_process_status (
    batch_process_status_id INTEGER PRIMARY KEY,
    name                    TEXT
);

CREATE TABLE IF NOT EXISTS mer_batch_process_error (
    batch_process_error_id        INTEGER PRIMARY KEY,
    batch_process_id              INTEGER,
    error_id                      INTEGER,
    created_date                  TEXT,
    cleared_date                  TEXT,
    batch_process_error_status_id INTEGER,

    FOREIGN KEY (batch_process_id) REFERENCES mer_batch_process (
        batch_process_id
    ),
    FOREIGN KEY (error_id) REFERENCES mer_error (error_id),
    FOREIGN KEY (
        batch_process_error_status_id
    ) REFERENCES mer_batch_process_error_status (batch_process_error_status_id)
);

CREATE TABLE IF NOT EXISTS mer_batch_process_error_status (
    batch_process_error_status_id INTEGER PRIMARY KEY,
    name                          TEXT
);

CREATE TABLE IF NOT EXISTS mer_outbox_message (
    outbox_message_id       INTEGER PRIMARY KEY,
    source_batch_process_id INTEGER NOT NULL,
    payload                 TEXT NOT NULL,
    delivery_state_id       INTEGER NOT NULL,

    FOREIGN KEY (source_batch_process_id) REFERENCES mer_batch_process (
        batch_process_id
    ),
    FOREIGN KEY (delivery_state_id) REFERENCES mer_delivery_state (
        delivery_state_id
    )
);

CREATE TABLE IF NOT EXISTS mer_delivery_state (
    delivery_state_id INTEGER PRIMARY KEY,
    name              TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS steam_partner_highwatermark (
    partner_highwatermark_id INTEGER PRIMARY KEY,
    value                    TEXT NOT NULL,

    -- 1:1 relationship with mer_batch_process_id
    batch_process_id         INTEGER NOT NULL,

    FOREIGN KEY (batch_process_id) REFERENCES mer_batch_process (
        batch_process_id
    )
);

CREATE TABLE IF NOT EXISTS steam_app_type (
    steam_app_type_id INTEGER PRIMARY KEY,
    steam_name        TEXT
);

CREATE TABLE IF NOT EXISTS steam_detailed_sales_result (
    detailed_sales_result_id      INTEGER PRIMARY KEY,
    application_id                INTEGER NOT NULL,
    batch_process_id              TEXT,

    -- Steam: CPartnerFinancials_DetailedSalesResult
    partnerid                     INTEGER,
    partner_name                  TEXT,
    "date"                        TEXT,
    line_item_type                TEXT,
    package_id                    INTEGER,
    package_name                  TEXT,
    bundle_id                     INTEGER,
    bundle_name                   TEXT,
    app_id                        INTEGER,
    app_name                      TEXT,
    game_item_id                  INTEGER,
    game_item_description         TEXT,
    game_item_category            TEXT,
    package_sale_type             TEXT,
    key_request_id                INTEGER,
    key_request_notes             TEXT,
    game_code_id                  INTEGER,
    game_code_description         TEXT,
    territory_code_id             INTEGER,
    territory_code_description    TEXT,
    platform                      TEXT,
    country_code                  TEXT,
    country_name                  TEXT,
    region                        TEXT,
    base_price                    INTEGER,
    sale_price                    INTEGER,
    currency                      TEXT,
    combined_discount_id          INTEGER,
    combined_discount_name        TEXT,
    total_discount_percentage     INTEGER,
    -- Array of objects with discountid, discount_description, discount_group,
    -- and discount_percentage. NULL denotes unavailable component metadata.
    discounts_json                TEXT CHECK (
        discounts_json IS NULL
        OR (json_valid(discounts_json) AND json_type(discounts_json) = 'array')
    ),
    gross_units_sold              INTEGER,
    gross_units_returned          INTEGER,
    gross_sales_usd               TEXT,
    gross_returns_usd             TEXT,
    net_tax_usd                   TEXT,
    gross_units_activated         INTEGER,
    view_grant_partnerid          INTEGER,
    view_grant_partner_name       TEXT,
    net_units_sold                INTEGER,
    net_sales_usd                 TEXT,
    avg_sale_price_usd            TEXT,
    primary_appid                 INTEGER,
    primary_app_name              TEXT,
    additional_revenue_share_tier INTEGER,

    FOREIGN KEY (application_id) REFERENCES mer_application (application_id),
    FOREIGN KEY (batch_process_id) REFERENCES mer_batch_process (
        batch_process_id
    )
);
