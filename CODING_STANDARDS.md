# Coding Standards: Parameter Naming Conventions

## Overview

This document establishes comprehensive coding standards for JSON parameter naming conventions in the WWE Universe Manager application. These standards ensure consistency between frontend parameter serialization and backend Tauri command expectations, preventing parameter serialization issues and maintaining type safety across the application.

## Core Principles

### 1. Tauri Parameter Conversion Understanding

**CRITICAL: Tauri automatically converts between snake_case (Rust) and camelCase (JavaScript)!**

This is the fundamental principle that guides all parameter naming:
- **Backend Rust functions**: Use snake_case parameters (`wrestler_id: i32`)
- **Frontend JavaScript/JSON**: Use camelCase parameters (`"wrestlerId": 1`)
- **Tauri Framework**: Handles automatic conversion between the two

### 2. Frontend-Backend Parameter Conversion

Frontend parameter names in `serde_json::json!` serialization MUST use camelCase, which Tauri automatically converts to snake_case for backend consumption.

**Correct Pattern:**
```rust
// Frontend (types.rs, components) - Use camelCase
let params = serde_json::json!({
    "wrestlerId": wrestler_id,     // Tauri converts to wrestler_id
    "showId": show_id,             // Tauri converts to show_id
    "eventName": event_name        // Tauri converts to event_name
});

// Backend (lib.rs, db.rs) - Use snake_case
#[tauri::command]
pub fn assign_wrestler_to_show(wrestler_id: i32, show_id: i32) -> Result<String, String> {
    // Implementation - receives snake_case parameters from Tauri
}
```

**Incorrect Pattern:**
```rust
// Frontend - WRONG (snake_case)
let params = serde_json::json!({
    "wrestler_id": wrestler_id,    // ❌ Should be "wrestlerId"
    "show_id": show_id,            // ❌ Should be "showId" 
    "event_name": event_name       // ❌ Should be "eventName"
});
```

## Parameter Naming Patterns

### Standard ID Parameters
All entity ID parameters follow these patterns:

| Entity Type | Frontend Parameter (camelCase) | Backend Parameter (snake_case) | Example Value |
|-------------|--------------------------------|--------------------------------|---------------|
| Wrestler    | `"wrestlerId"`                 | `wrestler_id`                  | `1`, `42`     |
| Title       | `"titleId"`                    | `title_id`                     | `5`, `13`     |
| Show        | `"showId"`                     | `show_id`                      | `2`, `8`      |
| Promotion   | `"promotionId"`                | `promotion_id`                 | `1`, `3`      |
| Match       | `"matchId"`                    | `match_id`                     | `7`, `21`     |
| User        | `"userId"`                     | `user_id`                      | `4`, `15`     |

### Compound Parameters
Multi-word parameters follow camelCase in frontend, snake_case in backend:

| Parameter Type | Frontend (camelCase) | Backend (snake_case) | 
|----------------|---------------------|---------------------|
| New wrestler ID | `"newWrestlerId"` | `new_wrestler_id` |
| Event name | `"eventName"` | `event_name` |
| Event location | `"eventLocation"` | `event_location` |
| Change method | `"changeMethod"` | `change_method` |
| Real name | `"realName"` | `real_name` |
| Debut year | `"debutYear"` | `debut_year` |
| Match type | `"matchType"` | `match_type` |
| Title type | `"titleType"` | `title_type` |

### Boolean Parameters
Boolean parameters follow the same camelCase/snake_case conversion:

| Parameter Type | Frontend (camelCase) | Backend (snake_case) |
|----------------|---------------------|---------------------|
| Active status | `"isActive"` | `is_active` |
| Title match flag | `"isTitleMatch"` | `is_title_match` |
| Current holder | `"isCurrent"` | `is_current` |

## Implementation Guidelines

### Frontend Components (`src/components/`)

When calling Tauri commands from Leptos components, always use camelCase parameter names:

```rust
// ✅ CORRECT - Frontend uses camelCase
use wasm_bindgen::prelude::*;

let wrestler_id = wrestler.id;
let result = invoke(
    "get_wrestler_by_id",
    serde_wasm_bindgen::to_value(&serde_json::json!({
        "wrestlerId": wrestler_id  // Tauri converts to wrestler_id
    })).unwrap()
).await;
```

```rust
// ❌ INCORRECT - Don't use snake_case in frontend
let result = invoke(
    "get_wrestler_by_id", 
    serde_wasm_bindgen::to_value(&serde_json::json!({
        "wrestler_id": wrestler_id  // Wrong: Tauri expects camelCase
    })).unwrap()
).await;
```

### Service Layer (`src/services/`)

Service layer functions must use camelCase in frontend parameter serialization:

```rust
// ✅ CORRECT - Frontend service uses camelCase
pub async fn update_title_holder(
    title_id: i32,
    new_wrestler_id: i32,
    event_name: String,
    event_location: String,
    change_method: String,
) -> Result<String, JsValue> {
    let result = invoke(
        "update_title_holder",
        serde_wasm_bindgen::to_value(&serde_json::json!({
            "titleId": title_id,           // Tauri converts to title_id
            "newWrestlerId": new_wrestler_id, // Tauri converts to new_wrestler_id
            "eventName": event_name,       // Tauri converts to event_name
            "eventLocation": event_location, // Tauri converts to event_location
            "changeMethod": change_method  // Tauri converts to change_method
        })).unwrap(),
    ).await;
    
    // Handle result...
}
```

### Type Definitions (`src/types.rs`)

Type definitions for API calls must use camelCase parameter naming:

```rust
// ✅ CORRECT - Frontend uses camelCase
pub async fn assign_wrestler_to_show(show_id: i32, wrestler_id: i32) -> Result<String, JsValue> {
    invoke(
        "assign_wrestler_to_show",
        serde_wasm_bindgen::to_value(&serde_json::json!({
            "showId": show_id,         // Tauri converts to show_id
            "wrestlerId": wrestler_id  // Tauri converts to wrestler_id
        })).unwrap(),
    ).await
    .map_err(|e| JsValue::from_str(&format!("Failed to assign wrestler to show: {:?}", e)))?
    .as_string()
    .ok_or_else(|| JsValue::from_str("Invalid response type"))
}
```

### Backend Commands (`src-tauri/src/lib.rs`, `src-tauri/src/db.rs`)

Backend Tauri commands use snake_case parameters, which Tauri automatically converts from frontend camelCase:

```rust
// ✅ Backend command signature uses snake_case
#[tauri::command]
pub async fn assign_wrestler_to_show(
    app_handle: AppHandle,
    show_id: i32,       // Tauri converts from frontend "showId" 
    wrestler_id: i32,   // Tauri converts from frontend "wrestlerId"
) -> Result<String, String> {
    let db = get_db_connection(&app_handle)?;
    db::internal_assign_wrestler_to_show(&mut db.get().unwrap(), show_id, wrestler_id)
}
```

## How Tauri Parameter Conversion Works

### The Conversion Process

1. **Frontend sends camelCase**: `{"wrestlerId": 123, "showId": 456}`
2. **Tauri receives and converts**: `wrestler_id: 123, show_id: 456`  
3. **Backend function receives snake_case**: Parameters match Rust conventions
4. **Response flows back**: Return values follow same conversion rules

### Why This Matters

- **JavaScript Convention**: Frontend follows JavaScript camelCase standards
- **Rust Convention**: Backend follows Rust snake_case standards  
- **Automatic Translation**: Tauri handles the conversion seamlessly
- **Type Safety**: Both sides maintain their native conventions

## Common Mistakes and How to Avoid Them

### Mistake 1: Using snake_case in Frontend
**Problem:** Frontend uses `"wrestler_id"` but Tauri expects camelCase
**Solution:** Always use camelCase in JSON parameter objects
**Detection:** Runtime serialization error: `missing field 'wrestler_id'`

### Mistake 2: Using camelCase in Backend
**Problem:** Backend parameter defined as `wrestlerId: i32`
**Solution:** Backend should always use snake_case: `wrestler_id: i32`
**Detection:** Compilation error or parameter mismatch

### Mistake 3: Not Understanding the Conversion
**Problem:** Trying to match parameter names exactly between frontend and backend
**Solution:** Trust Tauri's conversion - use camelCase frontend, snake_case backend
**Example:** Frontend `"eventName"` automatically becomes backend `event_name`

## Testing Parameter Serialization

### Compilation Testing
Always verify that parameter naming changes don't break compilation:

```bash
# Run these commands after parameter changes
cargo check --workspace
cargo build --release
npm run build-css-prod
```

### Functional Testing
Test actual parameter serialization with running application:

1. **Window Management Tests:**
   - Open wrestler windows with `"wrestlerId"` parameter
   - Open title windows with `"titleId"` parameter
   - Verify data loads correctly

2. **CRUD Operation Tests:**
   - Test create operations with camelCase parameters
   - Test update operations with compound parameters like `"newWrestlerId"`
   - Test delete operations with ID parameters

3. **Error Handling Tests:**
   - Test with invalid parameter values
   - Verify error messages are helpful
   - Ensure parameter validation works

## Integration with Existing Architecture

### Database Layer Integration
Backend parameters should align with database column names (both use snake_case):

```sql
-- Database schema uses snake_case
CREATE TABLE wrestlers (
    id INTEGER PRIMARY KEY,
    real_name TEXT,
    debut_year INTEGER,
    -- etc.
);
```

```rust
// Backend parameters match database columns (snake_case)
pub fn update_wrestler_details(
    wrestler_id: i32,
    real_name: String, 
    debut_year: i32,  // Converted from frontend "debutYear"
) -> Result<String, String> {
    // Implementation matches database schema
}
```

### Diesel ORM Integration
Diesel struct fields use snake_case, backend parameters match:

```rust
#[derive(Queryable, Selectable)]
#[diesel(table_name = wrestlers)]
pub struct Wrestler {
    pub id: i32,
    pub real_name: Option<String>,    // snake_case field
    pub debut_year: Option<i32>,      // snake_case field
    // Fields use snake_case
}

// Backend parameters match Diesel struct fields
pub fn create_wrestler(
    real_name: String,     // From frontend "realName"
    debut_year: i32,       // From frontend "debutYear"
) -> Result<String, String> {
    // Parameter names match Diesel struct
}
```

## Code Review Checklist

When reviewing code that involves Tauri parameters, verify:

- [ ] **Frontend Naming:** Frontend uses camelCase in JSON (`"wrestlerId"`)
- [ ] **Backend Naming:** Backend uses snake_case parameters (`wrestler_id: i32`)
- [ ] **Conversion Trust:** Code relies on Tauri's automatic conversion
- [ ] **Consistency:** New parameters follow established patterns from this document
- [ ] **Documentation:** Parameter changes are reflected in relevant documentation
- [ ] **Testing:** Parameter serialization has been tested (compilation + functional)
- [ ] **Type Safety:** Parameter types are consistent between frontend and backend
- [ ] **Error Handling:** Parameter validation and error messages are appropriate

## Migration Guide

### Converting Existing snake_case Frontend Parameters

When updating existing incorrect snake_case frontend parameters to camelCase:

1. **Identify All Usages:** Use grep/ripgrep to find all frontend parameter instances
2. **Update Frontend First:** Change JSON parameter names from snake_case to camelCase
3. **Verify Backend Unchanged:** Ensure backend still uses snake_case
4. **Test Compilation:** Run full build process
5. **Test Functionality:** Verify runtime parameter serialization works
6. **Update Documentation:** Reflect changes in relevant docs

### Example Migration
```rust
// BEFORE (incorrect snake_case in frontend)
let params = serde_json::json!({
    "wrestler_id": wrestler_id,    // Wrong for frontend
    "event_name": event_name,      // Wrong for frontend  
    "change_method": method        // Wrong for frontend
});

// AFTER (correct camelCase in frontend)  
let params = serde_json::json!({
    "wrestlerId": wrestler_id,     // Correct - Tauri converts to wrestler_id
    "eventName": event_name,       // Correct - Tauri converts to event_name
    "changeMethod": method         // Correct - Tauri converts to change_method
});
```

## Troubleshooting Parameter Issues

### Common Error Patterns

**Serialization Error:**
```
Error: Failed to deserialize parameters: missing field `wrestler_id`
```
**Cause:** Frontend using snake_case `"wrestler_id"` but Tauri expecting camelCase
**Solution:** Update frontend to use camelCase: `"wrestlerId"`

**Unknown Field Error:**
```
Error: unknown field `wrestlerId`, expected `wrestler_id`
```
**Cause:** Backend parameter defined incorrectly or Tauri conversion not working
**Solution:** Ensure backend uses snake_case: `wrestler_id: i32`

**Type Mismatch Error:**
```
Error: invalid type: string "1", expected i32
```
**Cause:** Parameter type inconsistency between frontend and backend
**Solution:** Ensure parameter types match exactly

### Debugging Steps

1. **Check Frontend Naming:** Verify frontend JSON uses camelCase
2. **Check Backend Naming:** Verify backend parameters use snake_case  
3. **Check Parameter Types:** Ensure type consistency (i32, String, bool, etc.)
4. **Check Serialization:** Log JSON parameter objects before sending to backend
5. **Check Backend Logs:** Review Tauri command execution logs for parameter issues
6. **Test Isolation:** Test parameter serialization in isolation

## Parameter Naming Reference

### Quick Reference Table

| Concept | Frontend (camelCase) | Backend (snake_case) | Notes |
|---------|---------------------|---------------------|-------|
| Wrestler ID | `"wrestlerId"` | `wrestler_id` | Primary identifier |
| Title ID | `"titleId"` | `title_id` | Primary identifier |
| Show ID | `"showId"` | `show_id` | Primary identifier |
| Event Name | `"eventName"` | `event_name` | Multi-word parameter |
| New Wrestler ID | `"newWrestlerId"` | `new_wrestler_id` | Compound parameter |
| Is Active | `"isActive"` | `is_active` | Boolean flag |
| Real Name | `"realName"` | `real_name` | Optional field |
| Debut Year | `"debutYear"` | `debut_year` | Numeric field |

## Future Considerations

### Extending Parameter Standards
When adding new parameter types or patterns:

1. **Follow Conversion Pattern:** Use camelCase frontend, snake_case backend
2. **Document New Patterns:** Update this document with new parameter types
3. **Test Thoroughly:** Ensure Tauri conversion works for new patterns
4. **Communicate Changes:** Update team on new parameter standards

### Integration with New Frameworks
If integrating additional frameworks or libraries:

1. **Maintain Conversion:** Keep camelCase frontend, snake_case backend standard
2. **Document Integration:** Explain how new frameworks fit parameter standards
3. **Test Integration:** Verify parameter serialization across framework boundaries

## Summary

Following these coding standards ensures:

- **Native Conventions:** Frontend uses JavaScript camelCase, backend uses Rust snake_case
- **Automatic Conversion:** Tauri handles translation between naming conventions seamlessly
- **Type Safety:** Frontend and backend parameter expectations align through conversion
- **Maintainability:** Clear conversion patterns make parameter naming predictable
- **Debugging:** Understanding conversion makes troubleshooting parameter issues easier
- **Team Productivity:** Developers can rely on established patterns and Tauri's conversion

**Remember:** Parameter naming consistency relies on understanding Tauri's automatic conversion. Frontend uses camelCase, backend uses snake_case, and Tauri handles the translation. Always reference this document when working with parameters, and trust Tauri's conversion mechanism.