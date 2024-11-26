# prometheus-series-diff

This is a tool for diffing Prometheus series data. For example, if there are changes to the data due to a major update or minor version upgrade, such as Prometheus v3, you can use this to detect those changes.
In addition to Prometheus, you can also use it to investigate the impact on series due to changes in exporter application and replay rules.

![Github Action](https://github.com/watawuwu/prometheus-series-diff/workflows/Test/badge.svg)

## Getting Started

If you are running Prometheus in parallel or creating replicas, you can specify different endpoints as arguments to check the differences.
By default, the `start` and `end` parameters are set to 10 minutes ago and 5 minutes ago, respectively.

```shell
$ prometheus-series-diff http://prometheus1.example.com:9090 http://prometheus2.example.com:9090
```

You can also specify the start and end parameters individually.

```shell
$ prometheus-series-diff --from-start 2021-01-01T00:00:00Z --from-end 2021-01-02T00:05:00Z --to-start 2021-01-01T00:00:00Z --to-end 2021-01-02T00:05:00Z http://prometheus1.example.com:9090 http://prometheus.example.com:9090
```

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Authors

- Wataru Matsui <watawuwu@3bi.tech>
