# Development Notes

## Tenancy

I need to support each Customer as a Tenant. The customer could be a single site Church (with any number of regular services) or a multi-site church (again any number of services). 

Use PATH to hold the tenant eg 

```
        .nest(
            "/:Tenant/Song/SongCollection",
            sng::song_collection::create_routes(),
```

I think setup short codes for tenants eg StNics, StAnWyth (St Andrews Wythenshawe)

Put the tennant_id in every table.

## Authentication

Looks like I should use the crate auth-login. A user should be a valid email address and be valid for a single tenant (OR owner which allows all tenants). 

## user roles

Not yet
