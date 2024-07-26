pub const README: &str = "PAXX
res check:
    input: stdin
    output: OK/ERR <comment>

diff check:
    input:
        solve: file(args[2])
        ref: file(args[3])
    output: OK/ERR <comment>

config: config.yml";