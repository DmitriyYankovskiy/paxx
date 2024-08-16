pub const README: &str = r#"PAXX

---------------------------------------------------------------------------------

test gen:                               ALL
    input:
        arg[1] - test number
    output:
        stdout: test

solution:                                  ALL
    input: 
        stdin: test
    output:
        stdout: result

reference:                              ComparisonResults | AutoComparisonResults
    input: 
        stdin: test
    output:
        stdout: result

res check:                              CheckingResults
    input:
        stdin: solve result
    output:
        stdout: OK/WA <comment>

comparison:                             ComparisonResults
    input:
        arg[1] - path to: solve result
        arg[2] - path to: ref result
    output: 
        stdout: OK/WA <comment>


---------------------------------------------------------------------------------

config - config.yml:
    testing_type:
    
    --  ComparisonResults :
        running tests for the solution and the reference solution and comparing the results
    
    --  AutoComparisonResults :
        running tests for the solution and the reference solution and finding difference between the results
    
    --  CheckingResults :
        running tests for the solution and checking results
    
    --  Manual :
        solo running

commands: 
    init
    check
    build
    run [test count],
    solo
    catch [error count] [test limit]
    get [test number]
    pat:
     - gen
     - std [? path ?]
     - edit_cfg_c++_vscode 

unix_wait_status:
 - 139: Segmentation fault
 
"#;
