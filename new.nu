def main [day: int] {
    let path = $'.\src\bin\day($day)'
    
    if ($path | path exists) { 
        print $"(ansi yellow)❌ ($path) already exists, skipping(ansi reset)" 
    } else { 
        mkdir $path
        print $"(ansi green)✓ created ($path)(ansi reset)"
    }
    
    mut manifest = (open cargo.toml);
    let name = $"day($day)"
    let main = ($path | path join main.rs) 
    
    if ("bin" in $manifest) {
        if ($manifest.bin | where name =~ $name | is-empty) {
            $manifest.bin ++= { 
                name: $name, path: $main
            }
            $manifest | save -f cargo.toml;
            print $"(ansi green)✓ updated manifest(ansi reset)"
        } else {
            print $"(ansi yellow)❌ ($name) already exists in the manifest, skipping(ansi reset)"
        }
    } else {
        $manifest = ($manifest | insert bin [["name","path"];[$name, $main]])
        $manifest | save -f cargo.toml;
        print $"(ansi green)✓ created [[bin]] section and added ($day)(ansi reset)"
    }
    cd $path;
    "static RAW_INPUT: &str = include_str!(\"input.txt\");\n\nfn main() {\n\n}" | save main.rs;
    print $"(ansi green)✓ created default main.rs(ansi reset)"
    touch input.txt
    print $"(ansi green)✓ created empty input.txt(ansi reset)"
}