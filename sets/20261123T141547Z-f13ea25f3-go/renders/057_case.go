package main

import (
    "fmt"
    "io"
    "os"
    "os/exec"
    "path/filepath"
)

func truthy(x any) bool {
    switch v := x.(type) {
    case string:
        return v != ""
    case bool:
        return v
    case int64:
        return v != 0
    case nil:
        return false
    }
    return true
}

var fArgs []string

func argsList() []string {
    if len(fArgs) > 0 { return fArgs }
    if len(os.Args) > 1 { return os.Args[1:] }
    return nil
}

func paramAt(i int) string {
    a := argsList()
    if i >= 1 && i <= len(a) { return a[i-1] }
    return ""
}

func runCmd(c *exec.Cmd) int64 {
    c.Stdout = os.Stdout
    c.Stderr = io.Discard
    if err := c.Run(); err != nil {
        if ee, ok := err.(*exec.ExitError); ok { return int64(ee.ExitCode()) }
        return 1
    }
    return 0
}

func globMatch(pattern, s string) bool {
    ok, _ := filepath.Match(pattern, s)
    return ok
}

func main() {
    var filename string
    var st int64

    fmt.Println("=== Basic Case Statement Example ===");
    if (!(paramAt(1) != "start")) {
        fmt.Println("Starting the service...");
    } else if (!(paramAt(1) != "stop")) {
        fmt.Println("Stopping the service...");
    } else if (!(paramAt(1) != "restart")) {
        fmt.Println("Restarting the service...");
    } else {
        fmt.Println(fmt.Sprintf("Usage: %v {start|stop|restart}", paramAt(0)));
        os.Exit(int(1));
    }
    fmt.Println("=== Case Statement with Pattern Matching ===");
    filename = paramAt(2); // line 28
    // TODO(unsupported): strHasSuffix
    if truthy(0) {
        fmt.Println(fmt.Sprintf("Processing text file: %v", filename));
    // TODO(unsupported): strHasSuffix
    } else if truthy(0) {
        fmt.Println(fmt.Sprintf("Processing shell script: %v", filename));
    // TODO(unsupported): strHasSuffix
    } else if truthy(0) {
        fmt.Println(fmt.Sprintf("Processing Python file: %v", filename));
    } else {
        fmt.Println(fmt.Sprintf("Unknown file type: %v", filename));
    }
    fmt.Println("=== Case Statement with Multiple Patterns ===");
    if ((((!(paramAt(3) != "help")) || (!(paramAt(3) != "h"))) || (!(paramAt(3) != "-h"))) || (!(paramAt(3) != "--help"))) {
        fmt.Println("Help information:");
        fmt.Println("  start  - Start the service");
        fmt.Println("  stop   - Stop the service");
        fmt.Println("  status - Show service status");
    } else if ((((!(paramAt(3) != "status")) || (!(paramAt(3) != "s"))) || (!(paramAt(3) != "-s"))) || (!(paramAt(3) != "--status"))) {
        fmt.Println("Service status: Running");
    } else {
        fmt.Println(fmt.Sprintf("Unknown option: %v", paramAt(3)));
    }
    fmt.Println("=== Case Statement with Character Classes ===");
    switch { // line 65
        case globMatch("[0-9]", paramAt(4)):
            fmt.Println(fmt.Sprintf("Single digit: %v", paramAt(4)));
        case globMatch("[a-z]", paramAt(4)):
            fmt.Println(fmt.Sprintf("Lowercase letter: %v", paramAt(4)));
        case globMatch("[A-Z]", paramAt(4)):
            fmt.Println(fmt.Sprintf("Uppercase letter: %v", paramAt(4)));
        case globMatch("[0-9][0-9]", paramAt(4)):
            fmt.Println(fmt.Sprintf("Two digit number: %v", paramAt(4)));
        case globMatch("*", paramAt(4)):
            fmt.Println(fmt.Sprintf("Other character: %v", paramAt(4)));
        default:
            _ = 0;
    }
    fmt.Println("=== Case Statement with Default Action ===");
    if (!(paramAt(5) != "red")) {
        fmt.Println("Color is red");
    } else if (!(paramAt(5) != "green")) {
        fmt.Println("Color is green");
    } else if (!(paramAt(5) != "blue")) {
        fmt.Println("Color is blue");
    }
    fmt.Println("=== Case Statement with Commands ===");
    if (!(paramAt(6) != "ls")) {
        st = runCmd(exec.Command("ls", "-l", "-a"));
    } else if (!(paramAt(6) != "date")) {
        st = runCmd(exec.Command("date"));
    } else if (!(paramAt(6) != "pwd")) {
        st = runCmd(exec.Command("pwd"));
    } else if (!(paramAt(6) != "whoami")) {
        st = runCmd(exec.Command("whoami"));
    } else {
        fmt.Println("Available commands: ls, date, pwd, whoami");
        st = 0;
    }
    _ = st
    return;
}
// 3 construct(s) lowered to TODO markers