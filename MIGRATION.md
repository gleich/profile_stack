# v2.0.0

v2.0.0 of profile_stack is a 100% rewrite in rust. Because it is written in rust the run time has been reduced from ~40+ seconds to ~10 seconds. There are a few things that have changed:

## `url` is now required

### Before

```yaml
- name: Golang
  logo: go
  url: https://golang.org/
  color: '#7FD6EA'
  projects:
    - https://github.com/gleich/fgh
    - https://github.com/gleich/gh_fsync
```

### After

```yaml
- name: Golang
  logo: go
  url: https://golang.org/
  color: '#7FD6EA'
  projects:
    - url: https://github.com/gleich/fgh
    - url: https://github.com/gleich/gh_fsync
```

### Difference

```diff
-   - https://github.com/gleich/fgh
+   - url: https://github.com/gleich/fgh
-   - https://github.com/gleich/gh_fsync
+   - url: https://github.com/gleich/gh_fsync
```

## `logoColor` is now `logo_color`

### Before

```yaml
- name: Golang
  logo: go
  logoColor: '#000000'
  url: https://golang.org/
  color: '#7FD6EA'
  projects:
    - url: https://github.com/gleich/fgh
    - url: https://github.com/gleich/gh_fsync
```

### After

```yaml
- name: Golang
  logo: go
  logo_color: '#000000'
  url: https://golang.org/
  color: '#7FD6EA'
  projects:
    - url: https://github.com/gleich/fgh
    - url: https://github.com/gleich/gh_fsync
```

### Difference

```diff
- logoColor: '#000000'
+ logo_color: '#000000'
```
