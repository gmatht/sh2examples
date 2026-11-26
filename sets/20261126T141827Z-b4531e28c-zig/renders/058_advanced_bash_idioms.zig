const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

fn sh2Run(argv: []const []const u8) []const u8 {
    const io = sh2_io orelse return "";
    const result = std.process.run(std.heap.page_allocator, io, .{
        .argv = argv,
        .stdout_limit = .limited(1 << 20),
    }) catch return "";
    return std.mem.trimEnd(u8, result.stdout, "\n");
}

fn sh2ArrayIndex() []const u8 {
    sh2TODO("arrayIndex");
    return "";
}

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

fn sh2GetVar() []const u8 {
    sh2TODO("getVar");
    return "";
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}


fn process_data(init: std.process.Init, __args: []const []const u8) ![]const u8 {
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    _ = sh2Run(&[_][]const u8{"local", "data_type=", (if (__args.len >= 1) __args[1 - 1] else "")});
    _ = sh2Run(&[_][]const u8{"local", "value=", (if (__args.len >= 2) __args[2 - 1] else "")});
    if ((!!std.mem.eql(u8, sh2Env("data_type"), "string"))) {
        // TODO(unsupported): param → sh2.param
        // TODO(unsupported): param → sh2.param
        __sh2_casesubj_0 = std.fmt.allocPrint(std.heap.page_allocator, "{s}", .{sh2Param()}) catch "";
        if (((!!std.mem.eql(u8, __sh2_casesubj_0, "hello")) or (!!std.mem.eql(u8, __sh2_casesubj_0, "hi")))) {
            try stdout.print("{s}{s}\n", .{"  Greeting detected: ", value});
    try stdout.flush();
        } else if (((!!std.mem.eql(u8, __sh2_casesubj_0, "bye")) or (!!std.mem.eql(u8, __sh2_casesubj_0, "goodbye")))) {
            try stdout.print("{s}{s}\n", .{"  Farewell detected: ", value});
    try stdout.flush();
        } else {
            try stdout.print("{s}{s}\n", .{"  Unknown string: ", value});
    try stdout.flush();
        }
    } else if ((!!std.mem.eql(u8, sh2Env("data_type"), "number"))) {
        if (std.mem.eql(u8, value, sh2GetVar())) {
            // TODO(unsupported): exec → sh2.exec
            // TODO(unsupported): exec → sh2.exec
            if (sh2Truthy(sh2Exec())) {
                try stdout.print("{s}{s}\n", .{"  Even number: ", value});
    try stdout.flush();
            } else {
                try stdout.print("{s}{s}\n", .{"  Odd number: ", value});
    try stdout.flush();
            }
        } else {
            try stdout.print("{s}{s}\n", .{"  Invalid number: ", value});
    try stdout.flush();
        }
    } else {
        try stdout.print("{s}{s}\n", .{"  Unknown data type: ", sh2Env("data_type")});
    try stdout.flush();
    }
}
pub fn main(init: std.process.Init) !void {

    const __sh2_casesubj_0: []const u8 = "";
    var a: i64 = 0;
    var b: i64 = 0;
    var c: i64 = 0;
    var dir_count: []const u8 = "";
    var file_count: []const u8 = "";
    var file_info: []const u8 = "";
    var host: []const u8 = "";
    var i: i64 = 0;
    var j: i64 = 0;
    var letter: []const u8 = "";
    var letters: [1024]i64 = undefined;
    var letters_len: usize = 0;
    var matrix_0_0_: []const u8 = "";
    var matrix_0_1_: []const u8 = "";
    var matrix_0_2_: []const u8 = "";
    var matrix_1_0_: []const u8 = "";
    var matrix_1_1_: []const u8 = "";
    var matrix_1_2_: []const u8 = "";
    var matrix_2_0_: []const u8 = "";
    var matrix_2_1_: []const u8 = "";
    var matrix_2_2_: []const u8 = "";
    var num: []const u8 = "";
    var numbers: [1024]i64 = undefined;
    var numbers_len: usize = 0;
    var port: i64 = 0;
    var result: []const u8 = "";
    var user: []const u8 = "";
    var value: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try stdout.print("{s}\n", .{"=== Advanced Bash Idioms Examples ==="});
    try stdout.flush();
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 7
    try stdout.print("{s}\n", .{"1. Nested loops with conditional logic and array manipulation:"});
    try stdout.flush();
    numbers_len = 0; // line 11
    numbers[numbers_len] = 1;
    numbers_len += 1;
    numbers[numbers_len] = 2;
    numbers_len += 1;
    numbers[numbers_len] = 3;
    numbers_len += 1;
    numbers[numbers_len] = 4;
    numbers_len += 1;
    numbers[numbers_len] = 5;
    numbers_len += 1;
    letters_len = 0; // line 12
    letters[letters_len] = sh2ToInt("a");
    letters_len += 1;
    letters[letters_len] = sh2ToInt("b");
    letters_len += 1;
    letters[letters_len] = sh2ToInt("c");
    letters_len += 1;
    letters[letters_len] = sh2ToInt("d");
    letters_len += 1;
    letters[letters_len] = sh2ToInt("e");
    letters_len += 1;
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    {
        const sh2_items0 = [_][]const u8{sh2Param()};
        for (sh2_items0[0..]) |sh2_v1| {
            num = sh2_v1;
            // TODO(unsupported): param → sh2.param
            // TODO(unsupported): param → sh2.param
            {
                const sh2_items2 = [_][]const u8{sh2Param()};
                for (sh2_items2[0..]) |sh2_v3| {
                    letter = sh2_v3;
                    if (((sh2ToInt(num) > 3) and !std.mem.eql(u8, letter, "c"))) {
                        try stdout.print("{s}{s}{s}{s}{s}\n", .{"  Number ", num, " with letter ", letter, " (filtered)"});
    try stdout.flush();
                    }
                }
            }
        }
    }
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 20
    try stdout.print("{s}\n", .{"2. Function with nested case statements and parameter expansion:"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"process_data", "string", "Hello"});
    _ = sh2Run(&[_][]const u8{"process_data", "string", "Bye"});
    _ = sh2Run(&[_][]const u8{"process_data", "number", "42"});
    _ = sh2Run(&[_][]const u8{"process_data", "number", "17"});
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 63
    try stdout.print("{s}\n", .{"3. Complex conditional with command substitution and arithmetic:"});
    try stdout.flush();
    file_count = sh2Capture(); // line 67
    dir_count = sh2Capture(); // line 68
    if (((sh2ToInt(file_count) > 0) and (sh2ToInt(dir_count) > 1))) {
        if ((file_count > dir_count)) {
            try stdout.print("{s}{s}{s}{s}{s}\n", .{"  More files (", file_count, ") than directories (", dir_count, ")"});
    try stdout.flush();
        } else {
            if ((file_count == dir_count)) {
                try stdout.print("{s}{s}{s}{s}{s}\n", .{"  Equal count: ", file_count, " files and ", dir_count, " directories"});
    try stdout.flush();
            } else {
                try stdout.print("{s}{s}{s}{s}{s}\n", .{"  More directories (", dir_count, ") than files (", file_count, ")"});
    try stdout.flush();
            }
        }
    } else {
        try stdout.print("{s}\n", .{"  Insufficient items for comparison"});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 81
    try stdout.print("{s}\n", .{"4. Nested here-documents with parameter expansion:"});
    try stdout.flush();
    user = "admin";
    host = "localhost";
    port = 22;
    try stdout.print("{s}", .{"    SSH Configuration:\n    $(cat <<'INNER'\n        User: $user\n        Host: $host\n        Port: $port\n        Status: $(ping -c 1 $host >/dev/null 2>&1 && echo \"Online\" || echo \"Offline\")\nINNER\n    )\n"});
    try stdout.flush();
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 99
    try stdout.print("{s}\n", .{"5. Array processing with nested loops and conditional logic:"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"declare", "-A", "matrix"}); // line 103
    matrix_0_0_ = "1"; // line 104
    matrix_0_1_ = "2"; // line 104
    matrix_0_2_ = "3"; // line 104
    matrix_1_0_ = "4"; // line 105
    matrix_1_1_ = "5"; // line 105
    matrix_1_2_ = "6"; // line 105
    matrix_2_0_ = "7"; // line 106
    matrix_2_1_ = "8"; // line 106
    matrix_2_2_ = "9"; // line 106
    {
        const sh2_items4 = [_]i64{0, 1, 2};
        for (sh2_items4[0..]) |sh2_v5| {
            i = sh2_v5;
            {
                const sh2_items6 = [_]i64{0, 1, 2};
                for (sh2_items6[0..]) |sh2_v7| {
                    j = sh2_v7;
                    // TODO(unsupported): arrayIndex → sh2.arrayIndex
                    // TODO(unsupported): arrayIndex → sh2.arrayIndex
                    value = sh2ArrayIndex();
                    if ((sh2ToInt(value) > 5)) {
                        try stdout.print("{s}{s}{s}", .{"  [", value, "] "});
    try stdout.flush();
                    } else {
                        try stdout.print("{s}{s}{s}", .{"  ", value, " "});
    try stdout.flush();
                    }
                }
            }
            try stdout.print("{s}\n", .{""});
    try stdout.flush();
        }
    }
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 119
    try stdout.print("{s}\n", .{"6. Process substitution with nested commands and error handling:"});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"  First word: ", sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"  Last word: ", sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"  Middle: ", sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"  Middle: ", sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"  Uppercase: ", sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"  Lowercase: ", sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"  Capitalize: ", sh2Param()});
    try stdout.flush();
    try stdout.print("{s}\n", .{""});
    try stdout.flush();
    try stdout.print("{s}\n", .{"11. Complex arithmetic with nested expressions:"});
    try stdout.flush();
    a = 10;
    b = 5;
    c = 3;
    result = sh2IntStr((((a + b) * c) - @divTrunc(@rem(a, b), c)));
    try stdout.print("{s}\n", .{"  Expression: (a + b) * c - (a % b) / c"});
    try stdout.flush();
    try stdout.print("{s}{d}{s}{d}{s}{d}\n", .{"  Values: a=", a, ", b=", b, ", c=", c});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"  Result: ", result});
    try stdout.flush();
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    if (sh2Truthy(sh2Exec())) {
        try stdout.print("{s}\n", .{"  Complex condition met: a > b AND (b < c OR a is even)"});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{""});
    try stdout.flush();
    try stdout.print("{s}\n", .{"12. Nested command substitution with error handling:"});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"  Current directory: ", sh2Capture()});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"  Parent directory: ", sh2Capture()});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"  Home directory: ", sh2Capture()});
    try stdout.flush();
    file_info = sh2Capture();
    try stdout.print("{s}{s}\n", .{"  File info: ", file_info});
    try stdout.flush();
    try stdout.print("{s}\n", .{""});
    try stdout.flush();
    try stdout.print("{s}\n", .{"=== Advanced Bash Idioms Examples Complete ==="});
    try stdout.flush();
    _ = &host;
    _ = &i;
    _ = &j;
    _ = &letters;
    _ = &matrix_0_0_;
    _ = &matrix_0_1_;
    _ = &matrix_0_2_;
    _ = &matrix_1_0_;
    _ = &matrix_1_1_;
    _ = &matrix_1_2_;
    _ = &matrix_2_0_;
    _ = &matrix_2_1_;
    _ = &matrix_2_2_;
    _ = &numbers;
    _ = &port;
    _ = &user;
}
// 26 construct(s) lowered to TODO markers