# Developer Reference: Parameter Naming Quick Guide

## Quick Reference

This document provides a quick reference for developers working on the WWE Universe Manager project, specifically focusing on parameter naming patterns and Tauri's automatic parameter conversion.

## IMPORTANT: Tauri Parameter Conversion

**Key Learning:** Tauri automatically converts snake_case Rust parameters to camelCase for JavaScript!

- **Backend (Rust)**: Use snake_case → `wrestler_id: i32`
- **Frontend (JavaScript/Leptos)**: Use camelCase → `"wrestlerId": 1`

**Critical Understanding:** This was a major learning from our recent debugging. The user corrected us that Tauri expects camelCase in frontend JSON, not snake_case. Always use camelCase like "wrestlerId" in frontend calls, and snake_case like `wrestler_id` in backend Rust functions.

## Parameter Naming Cheat Sheet

### Frontend JavaScript Parameter Names
Copy-paste these exact parameter names for frontend calls:

```json
{
    "wrestlerId": 1,
    "titleId": 5,
    "showId": 2,
    "promotionId": 1,
    "matchId": 7,
    "userId": 4,
    "newWrestlerId": 3,
    "eventName": "WrestleMania",
    "eventLocation": "MetLife Stadium",
    "changeMethod": "Pinfall Victory",
    "realName": "John Smith",
    "debutYear": 2020,
    "isActive": true,
    "isTitleMatch": false,
    "isCurrent": true
}
```

### Backend Rust Parameter Names
These are the corresponding Rust parameter names in Tauri commands:

```rust
// Tauri command parameters use snake_case
fn example_command(
    wrestler_id: i32,
    title_id: i32,
    show_id: i32,
    event_name: String,
    event_location: String,
    change_method: String,
    is_active: bool,
    is_title_match: bool
) -> Result<String, String>
```

### Frontend-Backend Parameter Mapping

| Frontend Context | Frontend Parameter (camelCase) | Backend Parameter (snake_case) | Backend Command | Type |
|------------------|-------------------------------|--------------------------------|-----------------|------|
| Opening wrestler window | `"wrestlerId"` | `wrestler_id` | `open_wrestler_window` | `Option<String>` |
| Opening title window | `"titleId"` | `title_id` | `open_title_window` | `Option<String>` |
| Assigning wrestler to show | `"showId"`, `"wrestlerId"` | `show_id`, `wrestler_id` | `assign_wrestler_to_show` | `i32`, `i32` |
| Removing wrestler from show | `"showId"`, `"wrestlerId"` | `show_id`, `wrestler_id` | `remove_wrestler_from_show` | `i32`, `i32` |
| Updating title holder | `"titleId"`, `"newWrestlerId"`, `"eventName"`, `"eventLocation"`, `"changeMethod"` | `title_id`, `new_wrestler_id`, `event_name`, `event_location`, `change_method` | `update_title_holder` | `i32`, `i32`, `String`, `String`, `String` |
| Getting wrestlers for show | `"showId"` | `show_id` | `get_wrestlers_for_show` | `i32` |
| Getting matches for show | `"showId"` | `show_id` | `get_matches_for_show` | `i32` |

## Common Code Patterns

### Component Parameter Passing

**Opening Windows:**
```rust
// ✅ CORRECT - Wrestler window (camelCase for frontend)
let wrestler_id = wrestler.id.to_string();
invoke("open_wrestler_window", serde_wasm_bindgen::to_value(&serde_json::json!({
    "wrestlerId": wrestler_id  // Tauri converts to snake_case automatically
})).unwrap()).await;

// ✅ CORRECT - Title window (camelCase for frontend)
let title_id = title.id.to_string();
invoke("open_title_window", serde_wasm_bindgen::to_value(&serde_json::json!({
    "titleId": title_id  // Tauri converts to snake_case automatically
})).unwrap()).await;
```

**Show Roster Management:**
```rust
// ✅ CORRECT - Assign wrestler to show (camelCase for frontend)
invoke("assign_wrestler_to_show", serde_wasm_bindgen::to_value(&serde_json::json!({
    "showId": show_id,      // Tauri converts to show_id
    "wrestlerId": wrestler_id  // Tauri converts to wrestler_id
})).unwrap()).await;

// ✅ CORRECT - Remove wrestler from show (camelCase for frontend)
invoke("remove_wrestler_from_show", serde_wasm_bindgen::to_value(&serde_json::json!({
    "showId": show_id,      // Tauri converts to show_id
    "wrestlerId": wrestler_id  // Tauri converts to wrestler_id
})).unwrap()).await;
```

### Service Layer Patterns

**Title Management:**
```rust
// ✅ CORRECT - Update title holder with full event details (camelCase for frontend)
pub async fn update_title_holder(
    title_id: i32,
    new_wrestler_id: i32,
    event_name: String,
    event_location: String,
    change_method: String,
) -> Result<String, JsValue> {
    invoke(
        "update_title_holder",
        serde_wasm_bindgen::to_value(&serde_json::json!({
            "titleId": title_id,           // Tauri converts to title_id
            "newWrestlerId": new_wrestler_id, // Tauri converts to new_wrestler_id
            "eventName": event_name,       // Tauri converts to event_name
            "eventLocation": event_location, // Tauri converts to event_location
            "changeMethod": change_method   // Tauri converts to change_method
        })).unwrap(),
    ).await
    .map_err(|e| JsValue::from_str(&format!("Failed to update title holder: {:?}", e)))?
    .as_string()
    .ok_or_else(|| JsValue::from_str("Invalid response type"))
}
```

**Data Fetching:**
```rust
// ✅ CORRECT - Fetch wrestlers for specific show (camelCase for frontend)
pub async fn fetch_wrestlers_for_show(show_id: i32) -> Result<Vec<Wrestler>, JsValue> {
    invoke(
        "get_wrestlers_for_show",
        serde_wasm_bindgen::to_value(&serde_json::json!({
            "showId": show_id  // Tauri converts to show_id
        })).unwrap(),
    ).await
    .map_err(|e| JsValue::from_str(&format!("Failed to fetch wrestlers for show: {:?}", e)))?
    .as_string()
    .and_then(|s| serde_json::from_str::<Vec<Wrestler>>(&s).ok())
    .ok_or_else(|| JsValue::from_str("Failed to parse wrestlers response"))
}
```

## Backend Command Reference

### Tauri Parameter Conversion Understanding

**How Tauri Works:**
1. Backend Rust functions use snake_case parameters
2. Tauri automatically converts snake_case to camelCase for JavaScript
3. Frontend must use camelCase in JSON parameters
4. Tauri converts camelCase back to snake_case before calling Rust function

### Current Tauri Commands with Parameter Signatures

```rust
// Backend Rust signatures (snake_case)
// Window Management
async fn open_wrestler_window(wrestler_id: Option<String>) -> Result<(), String>  // Frontend uses "wrestlerId"
async fn open_title_window(title_id: Option<String>) -> Result<(), String>        // Frontend uses "titleId"

// Show Roster Operations  
fn get_wrestlers_for_show(show_id: i32) -> Result<String, String>                // Frontend uses "showId"
fn assign_wrestler_to_show(show_id: i32, wrestler_id: i32) -> Result<String, String>  // Frontend uses "showId", "wrestlerId"
fn remove_wrestler_from_show(show_id: i32, wrestler_id: i32) -> Result<String, String> // Frontend uses "showId", "wrestlerId"

// Title Operations
fn update_title_holder(                                                          // Frontend uses camelCase equivalents:
    title_id: i32,         // "titleId"
    new_wrestler_id: i32,  // "newWrestlerId"
    event_name: String,    // "eventName"
    event_location: String, // "eventLocation"
    change_method: String  // "changeMethod"
) -> Result<String, String>

// Match Operations
fn get_matches_for_show(show_id: i32) -> Result<String, String>                 // Frontend uses "showId"
fn create_match(show_id: i32, match_name: String, match_type: String) -> Result<String, String> // Frontend uses "showId", "matchName", "matchType"
```

## Troubleshooting Guide

### Parameter Serialization Errors

**Error:** `missing field 'wrestler_id'`
**Cause:** Frontend using snake_case `"wrestler_id"` instead of camelCase `"wrestlerId"`
**Fix:** Change JSON key to `"wrestlerId"` (Tauri converts camelCase to snake_case automatically)

**Error:** `unknown field 'wrestlerId'` in backend
**Cause:** Backend parameter defined as something other than `wrestler_id`
**Fix:** Ensure backend Tauri command uses snake_case `wrestler_id: i32`

**Error:** `invalid type: string "1", expected i32`
**Cause:** Passing string where i32 expected, or vice versa
**Fix:** Check parameter type in backend command signature and convert appropriately

**Error:** `Failed to deserialize parameters`
**Cause:** Parameter name conversion mismatch
**Fix:** Use camelCase in frontend JSON, snake_case in backend Rust - let Tauri handle conversion

### Testing Parameter Changes

```bash
# Always run after parameter changes
cargo check --workspace
cargo build --release  
npm run build-css-prod

# Test with running application
npm run dev
```

## Migration Examples

### Before/After Parameter Fixes

**Window Management:**
```rust
// ❌ BEFORE (incorrect - using snake_case in frontend)
invoke("open_wrestler_window", serde_wasm_bindgen::to_value(&serde_json::json!({
    "wrestler_id": wrestler_id  // Wrong - Tauri expects camelCase
})).unwrap()).await;

// ✅ AFTER (correct - using camelCase in frontend)  
invoke("open_wrestler_window", serde_wasm_bindgen::to_value(&serde_json::json!({
    "wrestlerId": wrestler_id  // Correct - Tauri converts to snake_case
})).unwrap()).await;
```

**Title Operations:**
```rust
// ❌ BEFORE (incorrect - using snake_case in frontend)
invoke("update_title_holder", serde_wasm_bindgen::to_value(&serde_json::json!({
    "title_id": title_id,         // Wrong
    "new_wrestler_id": wrestler_id, // Wrong
    "event_name": event_name,     // Wrong  
    "event_location": location,   // Wrong
    "change_method": method       // Wrong
})).unwrap()).await;

// ✅ AFTER (correct - using camelCase in frontend)
invoke("update_title_holder", serde_wasm_bindgen::to_value(&serde_json::json!({
    "titleId": title_id,           // Correct - Tauri converts to title_id
    "newWrestlerId": wrestler_id,  // Correct - Tauri converts to new_wrestler_id
    "eventName": event_name,       // Correct - Tauri converts to event_name
    "eventLocation": location,     // Correct - Tauri converts to event_location
    "changeMethod": method         // Correct - Tauri converts to change_method
})).unwrap()).await;
```

## File Locations Reference

### Frontend Files That Use Parameters
- `src/components/wrestlers_list.rs` - Wrestler window management
- `src/components/title_details_window.rs` - Title operations, window management  
- `src/components/titles_list.rs` - Title window management
- `src/components/show_roster_management.rs` - Show roster operations
- `src/services/wrestler_api.rs` - Service layer API calls
- `src/types.rs` - Type definitions and API wrappers

### Backend Files That Define Parameters  
- `src-tauri/src/lib.rs` - Tauri command definitions
- `src-tauri/src/db.rs` - Database operation implementations

## Parameter Validation Checklist

When adding or modifying parameters:

- [ ] **Backend Convention**: Backend uses snake_case (wrestler_id)
- [ ] **Frontend Convention**: Frontend uses camelCase ("wrestlerId")
- [ ] **Tauri Conversion**: Trust Tauri's automatic snake_case ↔ camelCase conversion
- [ ] **Type Consistency**: Parameter types match between frontend and backend
- [ ] **Documentation Updated**: Changes reflected in relevant docs
- [ ] **Compilation Test**: `cargo check --workspace` passes
- [ ] **Functional Test**: Parameter serialization works in running app

## Common Parameter Patterns in Codebase

### ID Parameters
- Always use `{entity}Id` pattern in frontend (camelCase)
- Always use `{entity}_id` pattern in backend (snake_case)
- Backend expects `i32` for database IDs  
- Window management expects `Option<String>` for optional IDs

### Event Parameters  
- `eventName`: String describing the event (frontend)
- `event_name`: String describing the event (backend)
- `eventLocation`: String for event location (frontend)
- `event_location`: String for event location (backend)
- `changeMethod`: String describing how title changed hands (frontend)
- `change_method`: String describing how title changed hands (backend)

### Boolean Flags
- `isActive`: Entity active status (frontend)
- `is_active`: Entity active status (backend)
- `isTitleMatch`: Whether match is for a title (frontend)
- `is_title_match`: Whether match is for a title (backend)
- `isCurrent`: Current holder status (frontend)
- `is_current`: Current holder status (backend)

### Compound IDs
- `newWrestlerId`: When updating/changing wrestlers (frontend)
- `new_wrestler_id`: When updating/changing wrestlers (backend)
- `oldWrestlerId`: When tracking previous wrestlers (frontend)
- `old_wrestler_id`: When tracking previous wrestlers (backend)
- `winnerId`: Match winner identification (frontend)
- `winner_id`: Match winner identification (backend)

## Error Messages Reference

Common parameter-related error messages and their meanings:

| Error Message | Meaning | Fix |
|---------------|---------|-----|
| `missing field 'wrestler_id'` | Frontend using snake_case instead of camelCase | Use `"wrestlerId"` not `"wrestler_id"` |
| `invalid type: string, expected i32` | Type mismatch in parameter | Convert string to i32 or vice versa |
| `unknown field 'wrestlerId'` in backend | Backend parameter not defined as snake_case | Use snake_case in backend: `wrestler_id: i32` |
| `Failed to deserialize parameters` | General serialization failure | Check frontend uses camelCase, backend uses snake_case |

## Quick Setup for New Developers

1. **Read Documents:**
   - This file (DEVELOPER_REFERENCE.md) 
   - CODING_STANDARDS.md for comprehensive guidelines
   - CLAUDE.md for project architecture

2. **Understand Tauri Pattern:**
   - Backend Tauri commands use snake_case parameters
   - Frontend JavaScript uses camelCase in JSON
   - Tauri automatically converts between snake_case ↔ camelCase
   - When in doubt, check existing working examples

3. **Test Your Changes:**
   ```bash
   cargo check --workspace    # Compilation test
   npm run build-css-prod     # CSS build test  
   npm run dev               # Functional test
   ```

4. **Common First-Time Mistakes:**
   - Using snake_case in frontend instead of camelCase
   - Using camelCase in backend instead of snake_case
   - Not understanding Tauri's automatic parameter conversion
   - Not testing parameter serialization with running app

## Summary

This reference provides quick access to:
- Common parameter names and patterns used in the codebase
- Copy-paste code examples for typical scenarios  
- Troubleshooting guide for parameter-related errors
- Migration examples showing before/after fixes
- File locations for parameter-related code
- Testing procedures for parameter changes

**Remember:** When working with Tauri parameters:
- **Backend**: Use snake_case (wrestler_id: i32)
- **Frontend**: Use camelCase ("wrestlerId": wrestler_id)
- **Tauri**: Handles conversion automatically between snake_case ↔ camelCase

For comprehensive guidelines and detailed explanations, see `CODING_STANDARDS.md`.