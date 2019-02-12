# hone

Type safe rust library for building SQL query strings, inspired by [esqueleto(haskell)](https://github.com/bitemyapp/esqueleto).

## Status

Under development :).

## Example

```rust
let a = Query::<User>::from_by(|q, a| {
    let one = val_(1);
    let eq = eq_(a.user_id(), one);
    let q = q.where_(eq);

    q.return_(a.user_id())
});

assert_eq!(select(a.unwrap()).to_sql(),
            "SELECT User.user_id FROM User WHERE (User.user_id = 1)".to_string());
```

## Features

- [x] SELECT
- [ ] UPDATE
  - [x] UPDATE SET
  - [ ] UPDATE SET FROM
- [x] INSERT
  - [x] INSERT INTO
  - [x] INSERT INTO SELECT
- [x] DELETE

- WHERE
  - [x] eq(=) / not equal(<>)
  - [x] and / or 
  - [x] in / not in
  - [x] between
  - [x] is null / is not null

- ORDER BY 
  - [x] ASC/DESC

- [ ] GROUP BY 

- JOINs
  - [x] Inner
  - [x] Left
  - [x] Right
  
- [x] Limit
- [x] Offset

- Functions
  - [x] SUM
  - [x] AVG
  - [x] COUNT
