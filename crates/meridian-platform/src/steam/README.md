# Steam API reference

Reference for upstream contracts. Meridian implementation status belongs in
[Architecture: Current state](../../../../ARCHITECTURE.md#current-state).
Ordinary responses reproduce Valve's
published examples; constructed edge cases are labeled separately. JSON punctuation is repaired
where needed, with published values and field spellings preserved. Requests illustrate the
documented parameters; they are not captured traffic or Valve-provided request samples.
Request keys are placeholders. Nushell examples require the appropriate local credential to
authenticate. Keep credentials and
authenticated URLs out of logs, errors, persisted records, and outbound envelopes.

## App discovery

[GetPartnerAppListForWebAPIKey](https://partner.steamgames.com/doc/webapi/ISteamApps#GetPartnerAppListForWebAPIKey)
uses a publisher key without Financial permissions. Version 2 includes names.

| Query parameter | Type | Required |
| --- | --- | --- |
| `key` | string | yes |
| `type_filter` | comma-separated string | no |

Filters: `game`, `application`, `tool`, `demo`, `dlc`, `music`; omission returns all types.

```http
GET /ISteamApps/GetPartnerAppListForWebAPIKey/v2/?key=SYNTHETIC_PUBLISHER_KEY HTTP/1.1
Host: partner.steam-api.com
```

```nu
let query = ({key: "SYNTHETIC_PUBLISHER_KEY"} | url build-query)
http get $"https://partner.steam-api.com/ISteamApps/GetPartnerAppListForWebAPIKey/v2/?($query)"
```

**Valve's published response** ([source](https://partner.steamgames.com/doc/webapi/ISteamApps#GetPartnerAppListForWebAPIKey)):

```json
{
  "applist": {
    "apps": {
      "app": [
        {
          "appid": 500,
          "app_type": "game",
          "app_name": "Left 4 Dead"
        },
        {
          "appid": 222840,
          "app_type": "tool",
          "app_name": "Left 4 Dead Dedicated Server"
        },
        {
          "appid": 222860,
          "app_type": "tool",
          "app_name": "Left 4 Dead 2 Dedicated Server"
        }
      ]
    }
  }
}
```

For Meridian, resolve configured names against these records locally. An ambiguous match needs
explicit handling; selecting the first match can collect the wrong game's data.

## Financial requests

[IPartnerFinancialsService](https://partner.steamgames.com/doc/webapi/IPartnerFinancialsService)
uses a Financial API Group key with partner-wide scope. All methods below use GET, version
`v001`, and the `partner.steam-api.com` host.

| Method | Required query parameters | Optional query parameters |
| --- | --- | --- |
| `GetChangedDatesForPartner` | `key: string`, `highwatermark: uint64` | `include_view_grants: bool` |
| `GetDetailedSales` | `key: string`, `date: string`, `highwatermark_id: uint64` | `include_view_grants: bool` |
| `GetAppWishlistReporting` | `key: string`, `appid: uint32`, `date: string` | — |

### Changed dates

```http
GET /IPartnerFinancialsService/GetChangedDatesForPartner/v001/?key=SYNTHETIC_FINANCIAL_KEY&highwatermark=0&include_view_grants=false HTTP/1.1
Host: partner.steam-api.com
```

```nu
let query = ({
  key: "SYNTHETIC_FINANCIAL_KEY"
  highwatermark: "0"
  include_view_grants: false
} | url build-query)
http get $"https://partner.steam-api.com/IPartnerFinancialsService/GetChangedDatesForPartner/v001/?($query)"
```

**Valve's published response** ([source](https://partner.steamgames.com/doc/webapi/IPartnerFinancialsService#GetChangedDatesForPartner)):

```json
{
  "response": {
    "dates": [
      "2025/04/28",
      "2025/05/04",
      "2025/05/05",
      "2025/05/06",
      "2025/05/07",
      "2025/05/08"
    ],
    "result_highwatermark": "1841518117865"
  }
}
```

**Synthetic edge case — no changes:** request with `highwatermark=1841518117865`.

```json
{
  "response": {
    "dates": [],
    "result_highwatermark": "1841518117865"
  }
}
```

### Detailed sales

```http
GET /IPartnerFinancialsService/GetDetailedSales/v001/?key=SYNTHETIC_FINANCIAL_KEY&date=2025%2F05%2F01&highwatermark_id=0&include_view_grants=false HTTP/1.1
Host: partner.steam-api.com
```

```nu
let query = ({
  key: "SYNTHETIC_FINANCIAL_KEY"
  date: "2025/05/01"
  highwatermark_id: "0"
  include_view_grants: false
} | url build-query)
http get $"https://partner.steam-api.com/IPartnerFinancialsService/GetDetailedSales/v001/?($query)"
```

**Valve's published response** ([source](https://partner.steamgames.com/doc/webapi/IPartnerFinancialsService#GetDetailedSales)):

```json
{
  "response": {
    "results": [
      {
        "partnerid": "10500",
        "date": "2025/05/01",
        "line_item_type": "Package",
        "packageid": 123,
        "bundleid": 777,
        "package_sale_type": "Steam",
        "platform": "Windows",
        "country_code": "AU",
        "base_price": "3995",
        "sale_price": "1995",
        "currency": "AUD",
        "gross_units_sold": 10,
        "gross_units_returned": -2,
        "gross_sales_usd": "311.7201",
        "gross_returns_usd": "-25.5322",
        "net_tax_usd": "26.0431",
        "primary_appid": 999,
        "additional_revenue_share_tier": 0,
        "net_units_sold": 8,
        "net_sales_usd": "260.1448",
        "combined_discount_id": 8888,
        "total_discount_percentage": 50
      },
      {
        "partnerid": "10500",
        "date": "2025/05/01",
        "line_item_type": "Package",
        "packageid": 456,
        "package_sale_type": "Steam",
        "platform": "Windows",
        "country_code": "CA",
        "base_price": "5999",
        "sale_price": "5999",
        "currency": "CAD",
        "gross_units_sold": 100,
        "gross_units_returned": 0,
        "gross_sales_usd": "4309.0101",
        "gross_returns_usd": "0.0000",
        "net_tax_usd": "517.0812",
        "primary_appid": 1099,
        "additional_revenue_share_tier": 1,
        "viw_grant_partnerid": "5200",
        "net_units_sold": 100,
        "net_sales_usd": "3791.9288"
      },
      {
        "partnerid": "10500",
        "date": "2025/05/01",
        "line_item_type": "MicroTxn",
        "appid": 1099,
        "game_item_id": 1,
        "country_code": "US",
        "currency": "USD",
        "gross_units_sold": 2,
        "gross_units_returned": 0,
        "gross_sales_usd": "9.9800",
        "gross_returns_usd": "0.0000",
        "net_tax_usd": "0.9980",
        "primary_appid": 1099,
        "additional_revenue_share_tier": 1,
        "net_units_sold": 2,
        "net_sales_usd": "8.9820",
        "avg_sale_price_usd": "4.9900"
      },
      {
        "partnerid": "10500",
        "date": "2025/05/01",
        "line_item_type": "Package",
        "packageid": 123,
        "package_sale_type": "Retail",
        "platform": "Unknown",
        "country_code": "AE",
        "primary_appid": 999,
        "key_request_id": 7100,
        "gross_units_activated": 10
      }
    ],
    "key_request_info": [
      {
        "key_request_id": 7100,
        "key_request_notes": "Keys for Key Markets",
        "game_code_id": 842,
        "game_code_description": "Retail Distribution - Middle East & North Africa",
        "territory_code_id": 0,
        "territory_code_description": "UNUSED"
      }
    ],
    "package_info": [
      {
        "packageid": 123,
        "package_name": "Grate Expectations: Cheese Preparation Simulator - Deluxe Edition"
      },
      {
        "packageid": 456,
        "package_name": "Grate Expectations 2: Curd Your Loins - Launch Edition"
      }
    ],
    "app_info": [
      {
        "appid": 999,
        "app_name": "Grate Expectations: Cheese Preparation Simulator"
      },
      {
        "appid": 1099,
        "app_name": "Grate Expectations 2: Curd Your Loins"
      }
    ],
    "bundle_info": [
      {
        "bundleid": 777,
        "bundle_name": "Grate Expectations Deluxe Bundle"
      }
    ],
    "combined_discount_info": [
      {
        "combined_discount_id": 8888,
        "combined_discount_name": "Bundle Discount",
        "total_discount_percentage": 50,
        "discount_ids": [
          11111
        ]
      }
    ],
    "game_item_info": [
      {
        "appid": 1099,
        "game_item_id": 1,
        "game_item_description": "500 Grams of Whey",
        "game_item_category": "Essential Cheese Product"
      }
    ],
    "country_info": [
      {
        "country_code": "AE",
        "country_name": "United Arab Emirates",
        "region": "Middle East & North Africa"
      },
      {
        "country_code": "AU",
        "country_name": "Australia",
        "region": "Oceania"
      },
      {
        "country_code": "CA",
        "country_name": "Canada",
        "region": "North America"
      },
      {
        "country_code": "US",
        "country_name": "United States",
        "region": "North America"
      }
    ],
    "max_id": "29481179373"
  }
}
```

The published response spells one field `viw_grant_partnerid`; the field table documents
`view_grant_partnerid`. The spelling above is retained to distinguish the example from the contract.

**Synthetic edge case — pagination complete:** same date, `highwatermark_id=29481179373`.

```json
{
  "response": {
    "results": [],
    "max_id": "29481179373"
  }
}
```

Additional documented metadata includes `discount_info` and `partner_info`, omitted from Valve's example.

#### Key request metadata

`key_request_info` describes Steam product-key issuance batches. A Retail result's
`key_request_id` joins it to the batch metadata above; `gross_units_activated` counts activations.

| Field | Type | Meaning |
| --- | --- | --- |
| `key_request_id` | uint32 | Batch identifier referenced by the activation row. |
| `key_request_notes` | string | Notes entered for the key request; up to 4,000 characters. |
| `game_code_id` | int32 | Associated game-code identifier. |
| `game_code_description` | string | Game-code description; up to 1,024 characters. |
| `territory_code_id` | int32 | Associated territory-code identifier. |
| `territory_code_description` | string | Territory-code description; up to 1,024 characters. |

##### Possible finding: legacy game-code metadata

Documentation review dated 2026-11-01: searches for `game_code_id` and `game_code_description`
found references to the financial interface, its translations, and copies of that interface,
but no independent explanation or formal deprecation notice. The
[financial schema and example](https://partner.steamgames.com/doc/webapi/IPartnerFinancialsService#GetDetailedSales)
include both fields. The [Steam Keys guide](https://partner.steamgames.com/doc/features/keys#TaggingKeys)
describes batch tags without naming these fields or documenting a mapping to them.

Possible interpretation: the fields expose legacy or internal metadata. Deprecation is
unconfirmed; absence elsewhere in the documentation is insufficient evidence. Preserve the
documented fields, treat their meaning as opaque, and seek Valve clarification before relying
on them for classification or equating `game_code_id` with an AppID, package ID, or tag.

##### Valve's published example

The `key_request_info` excerpt from
[Valve's example response](https://partner.steamgames.com/doc/webapi/IPartnerFinancialsService#GetDetailedSales),
with values unchanged:

```json
{
  "key_request_info": [
    {
      "key_request_id": 7100,
      "key_request_notes": "Keys for Key Markets",
      "game_code_id": 842,
      "game_code_description": "Retail Distribution - Middle East & North Africa",
      "territory_code_id": 0,
      "territory_code_description": "UNUSED"
    }
  ]
}
```

##### Synthetic edge case — legacy key request

Constructed partial response illustrating Valve's documented legacy note. The identifier is
fictional; this is not a published Valve response. Other batch fields are omitted deliberately.

```json
{
  "key_request_info": [
    {
      "key_request_id": 300001,
      "key_request_notes": "Synthetic request"
    }
  ]
}
```

Valve documents the exact note `"Synthetic request"` for placeholder batches covering key ranges
created before January 2013.

#### Sales row categories

| Row category | Discriminator | Relevant identifiers |
| --- | --- | --- |
| Store package | `line_item_type=Package`, `package_sale_type=Steam` | `packageid`, optional `bundleid` |
| In-game purchase | `line_item_type=MicroTxn` | `appid`, `game_item_id` |
| Key activation | `line_item_type=Package`, `package_sale_type=Retail` | `packageid`, `key_request_id` |

Optional dimensions include `combined_discount_id` and `view_grant_partnerid`.
Money: `base_price`/`sale_price` are local-currency cents; `*_usd` values are decimal strings
with four fractional digits. Preserve precision. Fields vary by category; absence is not zero.
The upstream example quotes `max_id` despite its declared `int64` type. Treat 64-bit cursors
losslessly. Revenue-share tier values: `0`, `1`, `2` correspond to no bonus, 5%, 10%.

### Wishlists

```http
GET /IPartnerFinancialsService/GetAppWishlistReporting/v001/?key=SYNTHETIC_FINANCIAL_KEY&appid=440&date=2026-02-15 HTTP/1.1
Host: partner.steam-api.com
```

```nu
let query = ({
  key: "SYNTHETIC_FINANCIAL_KEY"
  appid: 440
  date: "2026-02-15"
} | url build-query)
http get $"https://partner.steam-api.com/IPartnerFinancialsService/GetAppWishlistReporting/v001/?($query)"
```

**Valve's published response** ([source](https://partner.steamgames.com/doc/webapi/IPartnerFinancialsService#GetAppWishlistReporting)):

```json
{
  "response": {
    "appid": 440,
    "date": "2026-02-15",
    "wishlist_summary": {
      "wishlist_adds": 30,
      "wishlist_deletes": 15,
      "wishlist_purchases": 10,
      "wishlist_gifts": 5,
      "wishlist_adds_windows": 22,
      "wishlist_adds_mac": 3,
      "wishlist_adds_linux": 5
    },
    "country_summary": [
      {
        "country_code": "CA",
        "country_name": "Canada",
        "region": "North America",
        "summary_actions": {
          "wishlist_adds": 20,
          "wishlist_deletes": 6,
          "wishlist_purchases": 9,
          "wishlist_gifts": 2,
          "wishlist_adds_windows": 15,
          "wishlist_adds_mac": 1,
          "wishlist_adds_linux": 4
        }
      },
      {
        "country_code": "FR",
        "country_name": "France",
        "region": "Western Europe",
        "summary_actions": {
          "wishlist_adds": 10,
          "wishlist_deletes": 9,
          "wishlist_purchases": 1,
          "wishlist_gifts": 3,
          "wishlist_adds_windows": 5,
          "wishlist_adds_mac": 2,
          "wishlist_adds_linux": 1
        }
      }
    ],
    "language_summary": [
      {
        "language": 0,
        "language_name": "English",
        "summary_actions": {
          "wishlist_adds": 18,
          "wishlist_deletes": 4,
          "wishlist_purchases": 7,
          "wishlist_gifts": 1,
          "wishlist_adds_windows": 14,
          "wishlist_adds_mac": 0,
          "wishlist_adds_linux": 4
        }
      },
      {
        "language": 2,
        "language_name": "French",
        "summary_actions": {
          "wishlist_adds": 12,
          "wishlist_deletes": 11,
          "wishlist_purchases": 3,
          "wishlist_gifts": 4,
          "wishlist_adds_windows": 8,
          "wishlist_adds_mac": 3,
          "wishlist_adds_linux": 1
        }
      }
    ],
    "time_generated": 1772580513,
    "app_min_date": "2010-01-01"
  }
}
```

**Synthetic edge case — outside the available date range:**

```json
{
  "response": {
    "app_min_date": "2010-01-01"
  }
}
```

## Collection contracts

Sales dates use Pacific Time; wishlist dates use GMT. Accepted date formats: `YYYY-MM-DD`,
`YYYY/MM/DD`, `YYYYMMDD`. Pass changed-date strings unchanged into sales requests.

Start the partner cursor at zero. For each changed date, restart its sales cursor at zero;
continue with `max_id` until it equals the requested cursor. Replace the entire date, including
deleted rows. Wishlist refreshes cover open dates; closed dates need no repeated retrieval.

Meridian's intended persistence boundary commits replacements, the partner checkpoint, and
outbox messages atomically. Page results must be complete before advancing that checkpoint.
HTTP failures and decode failures must not become successful empty results.
Define typed requests, responses, and errors
when adding methods to `SteamClient`; do not expose an arbitrary endpoint interface.
