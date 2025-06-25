This feature was introduced in version `1.4.0`. Currently, only one parameter is configurable.

# lines_limit

`lines_limit` is an integer that limits the maximum number of lines per commit. If this limit is exceeded, the application will not execute. The default value is `1000`.

You can configure this parameter using `gim config --lines-limit <LINES_LIMIT>`.