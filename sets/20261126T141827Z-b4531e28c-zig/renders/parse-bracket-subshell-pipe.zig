const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

fn sh2Assign() []const u8 {
    sh2TODO("assign");
    return "";
}

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var __sh2_tc0: []const u8 = "";
    _ = init;

    __sh2_tc0 = sh2Capture();
    // TODO(unsupported): assign → sh2.assign
    // TODO(unsupported): assign → sh2.assign
    _ = ((__sh2_tc0 .len != 0) and sh2Truthy(sh2Assign()));
}
// 2 construct(s) lowered to TODO markers