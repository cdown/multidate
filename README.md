# multidate | [![Tests](https://img.shields.io/github/actions/workflow/status/cdown/multidate/ci.yml?branch=master)](https://github.com/cdown/multidate/actions?query=branch%3Amaster)

multidate prints multiple timezones' dates/times with offsets from the local
time. Useful for people who work in global teams and often need to know what
time it is somewhere else.

## Usage

```
% multidate America/Los_Angeles America/New_York Europe/London Asia/Tel_Aviv Asia/Shanghai
              Local: Fri 2023-06-09 13:05

America/Los_Angeles: Thu 2023-06-08 22:05 (-15h)
   America/New_York: Fri 2023-06-09 01:05 (-12h)
      Europe/London: Fri 2023-06-09 06:05 (-7h)
      Asia/Tel_Aviv: Fri 2023-06-09 08:05 (-5h)
      Asia/Shanghai: Fri 2023-06-09 13:05 (+0h)
```
