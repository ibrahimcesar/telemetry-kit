# GNU Terry Pratchett - X-Clacks-Overhead

## What is this?

This project includes the `X-Clacks-Overhead: GNU Terry Pratchett` header in all HTTP requests to honor the memory of Sir Terry Pratchett, author of the Discworld series.

## The Story

In Terry Pratchett's novel "Going Postal," the clacks system (a semaphore/telegraph system) operators use special codes to keep messages alive in the system:

- **G**: Send the message on
- **N**: Do not log the message
- **U**: Turn the message around at the end of the line

When an operator died, their name would be transmitted as a special message with these codes, keeping them "alive" in the overhead of the clacks system.

## The Tradition

Following Terry Pratchett's death in 2015, the developer community started a tradition of including the `X-Clacks-Overhead: GNU Terry Pratchett` header in HTTP responses to keep his name alive in the overhead of the internet.

> "A man is not dead while his name is still spoken." - Going Postal, Terry Pratchett

## Implementation

### Rust SDK

All HTTP requests from the telemetry-kit Rust SDK to the ingestion server include this header:

```rust
// src/sync/client.rs
headers.insert("X-Clacks-Overhead", "GNU Terry Pratchett".parse().unwrap());
```

### Documentation Site

The fumadocs documentation site includes this header in all responses:

```javascript
// docs/next.config.mjs
async headers() {
  return [
    {
      source: '/:path*',
      headers: [
        {
          key: 'X-Clacks-Overhead',
          value: 'GNU Terry Pratchett',
        },
      ],
    },
  ];
}
```

## Learn More

- [gnuterrypratchett.com](http://www.gnuterrypratchett.com/) - The official GNU Terry Pratchett site
- [Reddit r/discworld](https://www.reddit.com/r/discworld/) - Discworld community
- [Going Postal](https://en.wikipedia.org/wiki/Going_Postal) - The novel that inspired this tradition

## How to Verify

You can verify the header is being sent:

### SDK Requests
```bash
# Use a packet capture tool or HTTP debugging proxy
# The header will be visible in all requests to the ingestion endpoint
```

### Documentation Site
```bash
curl -I https://telemetry-kit.dev
# Look for: X-Clacks-Overhead: GNU Terry Pratchett
```

Or in browser DevTools:
1. Open DevTools (F12)
2. Go to Network tab
3. Visit any page
4. Check the Response Headers
5. Look for `X-Clacks-Overhead: GNU Terry Pratchett`

## Why Include This?

1. **Honor a Legend**: Terry Pratchett's work brought joy to millions and influenced countless developers
2. **Community Tradition**: Joining a global tradition of keeping Terry's memory alive
3. **No Harm**: The header adds negligible overhead (< 50 bytes)
4. **Easter Egg**: A delightful discovery for Discworld fans inspecting HTTP headers

## Files Modified

- `src/sync/client.rs` - Added header to SDK sync requests
- `docs/next.config.mjs` - Added header to documentation site responses
- `README.md` - Documented the tradition in Acknowledgments
- `CLACKS.md` - This file

---

**GNU Terry Pratchett** 🎩

*"Do you not know that a man is not dead while his name is still spoken?"*
