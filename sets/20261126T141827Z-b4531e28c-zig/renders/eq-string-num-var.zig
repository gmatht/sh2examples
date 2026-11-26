const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

fn sh2GetVar() []const u8 {
    sh2TODO("getVar");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var i: i64 = 0;
    _ = init;

    {
        const sh2_items0 = [_]i64{1, 2, 3};
        for (sh2_items0[0..]) |sh2_v1| {
            i = sh2_v1;
            // TODO(unsupported): exec → sh2.exec
            // TODO(unsupported): exec → sh2.exec
            _ = ((!(i != 2)) and sh2Truthy(sh2Exec()));
        }
    }
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (std.mem.eql(u8, sh2GetVar(), sh2IntStr(0)) and sh2Truthy(sh2Exec()));
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (std.mem.eql(u8, sh2GetVar(), sh2IntStr(0)) and sh2Truthy(sh2Exec()));
}
// 6 construct(s) lowered to TODO markers