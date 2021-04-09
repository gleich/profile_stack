<!-- DO NOT REMOVE - contributor_list:data:start:["Matt-Gleich"]:end -->

# profile_stack ![GitHub release (latest by date)](https://img.shields.io/github/v/release/Matt-Gleich/profile_stack)

[![lint](https://github.com/Matt-Gleich/profile_stack/actions/workflows/lint.yml/badge.svg)](https://github.com/Matt-Gleich/profile_stack/actions/workflows/lint.yml)
[![build](https://github.com/Matt-Gleich/profile_stack/actions/workflows/build.yml/badge.svg)](https://github.com/Matt-Gleich/profile_stack/actions/workflows/build.yml)
[![test](https://github.com/Matt-Gleich/profile_stack/actions/workflows/test.yml/badge.svg)](https://github.com/Matt-Gleich/profile_stack/actions/workflows/test.yml)
[![deploy](https://github.com/Matt-Gleich/profile_stack/actions/workflows/deploy.yml/badge.svg)](https://github.com/Matt-Gleich/profile_stack/actions/workflows/deploy.yml)

🚀 Display your tech stack on your GitHub profile's README

## [🆕 v2.0.0 Migration](./MIGRATION.md)

## ✨ Example

Add the following to a file in `.github/workflows`:

```yml
name: stack

on:
  push:
    branches:
      - main

jobs:
  profile_stack:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: Matt-Gleich/profile_stack@master
```

Based on a [config file](#️-config) this GitHub action will generate a table showing technologies and projects you've used them in (doesn't have to be all, pick any):

| 💻 **Technology**                                                                                                                      | 🚀 **Projects**                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| -------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [![Golang](https://img.shields.io/static/v1?label=&message=Golang&color=7FD6EA&logo=go&logoColor=FFFFFF)](https://golang.org/)         | ![fgh](https://img.shields.io/static/v1?label=&message=fgh&color=000605&logo=github&logoColor=FFFFFF&labelColor=000605) ![gh_fsync](https://img.shields.io/static/v1?label=&message=gh_fsync&color=000605&logo=github&logoColor=FFFFFF&labelColor=000605) ![nuke](https://img.shields.io/static/v1?label=&message=nuke&color=000605&logo=github&logoColor=FFFFFF&labelColor=000605) ![logoru](https://img.shields.io/static/v1?label=&message=logoru&color=000605&logo=github&logoColor=FFFFFF&labelColor=000605) ![statuser](https://img.shields.io/static/v1?label=&message=statuser&color=000605&logo=github&logoColor=FFFFFF&labelColor=000605) |
| [![Python](https://img.shields.io/static/v1?label=&message=Python&color=3C78A9&logo=python&logoColor=FFFFFF)](https://www.python.org/) | ![profile_stack](https://img.shields.io/static/v1?label=&message=profile_stack&color=000605&logo=github&logoColor=FFFFFF&labelColor=000605) ![Contribution-Hat](https://img.shields.io/static/v1?label=&message=Contribution-Hat&color=000605&logo=github&logoColor=FFFFFF&labelColor=000605)                                                                                                                                                                                                                                                                                                                                                       |

You can see a live example at my repo: [github.com/Matt-Gleich/Matt-Gleich](https://github.com/Matt-Gleich/Matt-Gleich)

## ⚙️ Config

Configuration for the profile stack. Located by default in `stack.yml` at the root of your repository. Below is an example config:

```yml
- name: Golang
  logo: go
  url: https://golang.org/
  color: '#7FD6EA'
  projects:
    - url: https://github.com/Matt-Gleich/fgh
    - url: https://github.com/Matt-Gleich/gh_fsync
    - url: https://github.com/Matt-Gleich/nuke
    - url: https://github.com/Matt-Gleich/logoru
    - url: https://github.com/Matt-Gleich/statuser

- name: Python
  logo: python
  url: https://www.python.org/
  color: '#3C78A9'
  projects:
    - url: https://github.com/Matt-Gleich/profile_stack
    - url: https://github.com/Matt-Gleich/Contribution-Hat
```

So for each technology, there are the following fields you need to fill in:

| **Key**      | **Example Value**                                                                                       | **Description**                                                   | **Default** |
| ------------ | ------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- | ----------- |
| `name`       | Dart                                                                                                    | Name of the technology                                            | Required    |
| `logo`       | dart                                                                                                    | [Logo](https://simpleicons.org/) for the technology               | Required    |
| `url`        | https://flutter.dev/                                                                                    | URL for the technology                                            | Required    |
| `logo_color` | FFFFFF                                                                                                  | Hex color code for the logo color                                 | `#FFFFFF`   |
| `color`      | 52C0F2                                                                                                  | Hex color code for the background color                           | Required    |
| `projects`   | `- url: https://github.com/Matt-Gleich/Personal-Site` </br> `- url: https://github.com/Matt-Gleich/fgh` | List of GitHub project URLs or [project objects](#project-object) | Required    |

### Project object

You pass a list of YAML objects to the `projects` field.

| **Key** | **Example Value**                              | **Description**                    | **Default** |
| ------- | ---------------------------------------------- | ---------------------------------- | ----------- |
| `url`   | `https://github.com/Matt-Gleich/Personal-Site` | URL to a GitHub project            | Required    |
| `wip`   | `true`                                         | Mark a project as work-in-progress | `false`     |

## 🤖 Action Configuration

Here is an example config:

```yaml
name: stack

on:
  push:
    branches:
      - main

jobs:
  profile_stack:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: Matt-Gleich/profile_stack@master
        with:
          path: config/stack.yml
          badges: false
          technology_emoji: 👨🏻‍💻
          project_emoji: ✨
```

You can also configure the following when declaring your action:

| **Key**            | **Example Value** | **Description**                                                   | **Default** |
| ------------------ | ----------------- | ----------------------------------------------------------------- | ----------- |
| `path`             | config/stack.yml  | The path in your repository where the config file is located      | `stack.yml` |
| `badges`           | `false`           | Don't have badges, just plain old urls                            | `false`     |
| `technology_emoji` | 👨🏻‍💻                | The character to be displayed to the left of the Technology title | `💻`        |
| `project_emoji`    | ✨                | The character to be displayed to the left of the Project title    | `🚀`        |

## 🙌 Contributing

We would love to have you contribute! Please read the [contributing guide](CONTRIBUTING.md) before submitting a pull request. Thank you in advance!

<!-- prettier-ignore-start -->
<!-- DO NOT REMOVE - contributor_list:start -->
## 👥 Contributors


- **[@Matt-Gleich](https://github.com/Matt-Gleich)**

<!-- DO NOT REMOVE - contributor_list:end -->
<!-- prettier-ignore-end -->
