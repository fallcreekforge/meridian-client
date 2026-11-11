-- Create "mer_platform" table
CREATE TABLE `mer_platform` (`platform_id` integer NULL, `name` text NULL, PRIMARY KEY (`platform_id`));
-- Create "mer_error" table
CREATE TABLE `mer_error` (`error_id` integer NULL, `message` text NULL, `stack_trace` text NULL, PRIMARY KEY (`error_id`));
-- Create "mer_application" table
CREATE TABLE `mer_application` (`application_id` integer NULL, `steam_app_id` integer NULL, `steam_app_type_id` integer NULL, `steam_app_name` text NULL, `meridian_application_id` text NULL, PRIMARY KEY (`application_id`), CONSTRAINT `0` FOREIGN KEY (`steam_app_type_id`) REFERENCES `steam_app_type` (`steam_app_type_id`) ON UPDATE NO ACTION ON DELETE NO ACTION);
-- Create index "mer_application_steam_app_id" to table: "mer_application"
CREATE UNIQUE INDEX `mer_application_steam_app_id` ON `mer_application` (`steam_app_id`);
-- Create "mer_batch_process" table
CREATE TABLE `mer_batch_process` (`batch_process_id` integer NULL, `process_start` text NULL, `process_end` text NULL, `batch_process_type_id` integer NULL, `batch_process_status_id` integer NULL, `platform_id` integer NULL, PRIMARY KEY (`batch_process_id`), CONSTRAINT `0` FOREIGN KEY (`platform_id`) REFERENCES `mer_platform` (`platform_id`) ON UPDATE NO ACTION ON DELETE NO ACTION, CONSTRAINT `1` FOREIGN KEY (`batch_process_status_id`) REFERENCES `mer_batch_process_status` (`batch_process_status_id`) ON UPDATE NO ACTION ON DELETE NO ACTION, CONSTRAINT `2` FOREIGN KEY (`batch_process_type_id`) REFERENCES `mer_batch_process_type` (`batch_process_type_id`) ON UPDATE NO ACTION ON DELETE NO ACTION);
-- Create "mer_batch_process_type" table
CREATE TABLE `mer_batch_process_type` (`batch_process_type_id` integer NULL, `name` text NULL, PRIMARY KEY (`batch_process_type_id`));
-- Create "mer_batch_process_status" table
CREATE TABLE `mer_batch_process_status` (`batch_process_status_id` integer NULL, `name` text NULL, PRIMARY KEY (`batch_process_status_id`));
-- Create "mer_batch_process_error" table
CREATE TABLE `mer_batch_process_error` (`batch_process_error_id` integer NULL, `batch_process_id` integer NULL, `error_id` integer NULL, `created_date` text NULL, `cleared_date` text NULL, `batch_process_error_status_id` integer NULL, PRIMARY KEY (`batch_process_error_id`), CONSTRAINT `0` FOREIGN KEY (`batch_process_error_status_id`) REFERENCES `mer_batch_process_error_status` (`batch_process_error_status_id`) ON UPDATE NO ACTION ON DELETE NO ACTION, CONSTRAINT `1` FOREIGN KEY (`error_id`) REFERENCES `mer_error` (`error_id`) ON UPDATE NO ACTION ON DELETE NO ACTION, CONSTRAINT `2` FOREIGN KEY (`batch_process_id`) REFERENCES `mer_batch_process` (`batch_process_id`) ON UPDATE NO ACTION ON DELETE NO ACTION);
-- Create "mer_batch_process_error_status" table
CREATE TABLE `mer_batch_process_error_status` (`batch_process_error_status_id` integer NULL, `name` text NULL, PRIMARY KEY (`batch_process_error_status_id`));
-- Create "mer_outbox_message" table
CREATE TABLE `mer_outbox_message` (`outbox_message_id` integer NULL, `source_batch_process_id` integer NOT NULL, `payload` text NOT NULL, `delivery_state_id` integer NOT NULL, PRIMARY KEY (`outbox_message_id`), CONSTRAINT `0` FOREIGN KEY (`delivery_state_id`) REFERENCES `mer_delivery_state` (`delivery_state_id`) ON UPDATE NO ACTION ON DELETE NO ACTION, CONSTRAINT `1` FOREIGN KEY (`source_batch_process_id`) REFERENCES `mer_batch_process` (`batch_process_id`) ON UPDATE NO ACTION ON DELETE NO ACTION);
-- Create "mer_delivery_state" table
CREATE TABLE `mer_delivery_state` (`delivery_state_id` integer NULL, `name` text NOT NULL, PRIMARY KEY (`delivery_state_id`));
-- Create "steam_partner_highwatermark" table
CREATE TABLE `steam_partner_highwatermark` (`partner_highwatermark_id` integer NULL, `value` text NOT NULL, `batch_process_id` integer NOT NULL, PRIMARY KEY (`partner_highwatermark_id`), CONSTRAINT `0` FOREIGN KEY (`batch_process_id`) REFERENCES `mer_batch_process` (`batch_process_id`) ON UPDATE NO ACTION ON DELETE NO ACTION);
-- Create "steam_app_type" table
CREATE TABLE `steam_app_type` (`steam_app_type_id` integer NULL, `steam_name` text NULL, PRIMARY KEY (`steam_app_type_id`));
-- Create "steam_detailed_sales_result" table
CREATE TABLE `steam_detailed_sales_result` (`detailed_sales_result_id` integer NULL, `application_id` integer NOT NULL, `batch_process_id` text NULL, `partnerid` integer NULL, `partner_name` text NULL, `date` text NULL, `line_item_type` text NULL, `package_id` integer NULL, `package_name` text NULL, `bundle_id` integer NULL, `bundle_name` text NULL, `app_id` integer NULL, `app_name` text NULL, `game_item_id` integer NULL, `game_item_description` text NULL, `game_item_category` text NULL, `package_sale_type` text NULL, `key_request_id` integer NULL, `key_request_notes` text NULL, `game_code_id` integer NULL, `game_code_description` text NULL, `territory_code_id` integer NULL, `territory_code_description` text NULL, `platform` text NULL, `country_code` text NULL, `country_name` text NULL, `region` text NULL, `base_price` integer NULL, `sale_price` integer NULL, `currency` text NULL, `combined_discount_id` integer NULL, `combined_discount_name` text NULL, `total_discount_percentage` integer NULL, `discounts_json` text NULL, `gross_units_sold` integer NULL, `gross_units_returned` integer NULL, `gross_sales_usd` text NULL, `gross_returns_usd` text NULL, `net_tax_usd` text NULL, `gross_units_activated` integer NULL, `view_grant_partnerid` integer NULL, `view_grant_partner_name` text NULL, `net_units_sold` integer NULL, `net_sales_usd` text NULL, `avg_sale_price_usd` text NULL, `primary_appid` integer NULL, `primary_app_name` text NULL, `additional_revenue_share_tier` integer NULL, PRIMARY KEY (`detailed_sales_result_id`), CONSTRAINT `0` FOREIGN KEY (`batch_process_id`) REFERENCES `mer_batch_process` (`batch_process_id`) ON UPDATE NO ACTION ON DELETE NO ACTION, CONSTRAINT `1` FOREIGN KEY (`application_id`) REFERENCES `mer_application` (`application_id`) ON UPDATE NO ACTION ON DELETE NO ACTION, CHECK (
        discounts_json IS NULL
        OR (json_valid(discounts_json) AND json_type(discounts_json) = 'array')
    ));
