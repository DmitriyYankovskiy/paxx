use crate::{Flags, utils::file};



pub fn gen(path: &String, flags: &Flags) {
    file::create_file(path.clone(), GEN_PATTERN, flags);
}
pub fn std(path: &String, flags: &Flags) {
    file::create_file(path.clone(), STD_PATTERN, flags);
}
pub fn edit_cfg_cpp_vscode(path: &String, flags: &Flags) {
    file::create_file(path.clone(), EDIT_CFG_CPP_VSCODE, flags);
}
const GEN_PATTERN: &str = 
r#"#include<bits/stdc++.h>

using namespace std;

const int SEED = 's' + 'e' + 'e' + 'd';

int main(int argc, char *argv[]) {
    int test_number = atoi(argv[1]);
    mt19937 rnd(test_number + SEED);
    cout << rnd() << endl;
}
"#;
const STD_PATTERN: &str =
r#"#define PRAGMA

#ifdef PRAGMA
#pragma GCC optimize("O3,unroll-loops")
#endif

#include<bits/stdc++.h>

#ifdef PRAGMA
#pragma GCC target("avx2,bmi,bmi2,lzcnt,popcnt")
#endif

using namespace std;
using ll = long long;
using dbl = long double;

#define all(v) v.begin(), v.end()

#ifdef SOLO
#define debug(x) std::cout << #x << " = " << x << endl;
#else
#define debug(x);
#endif

const int INF = 1e9 + 10;

void solve() {

}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int t = 1;
#ifdef SOLO
    cin >> t;
#endif
    for (ll i = 0; i < t; ++i) {
        solve();
    }
#ifdef SOLO
    cout << '\n';
#endif
}
"#;


const EDIT_CFG_CPP_VSCODE: &str =
r#"
[*]
cpp_indent_braces=false
cpp_indent_multi_line_relative_to=innermost_parenthesis
cpp_indent_within_parentheses=indent
cpp_indent_preserve_within_parentheses=false
cpp_indent_case_labels=false
cpp_indent_case_contents=true
cpp_indent_case_contents_when_block=false
cpp_indent_lambda_braces_when_parameter=true
cpp_indent_goto_labels=one_left
cpp_indent_preprocessor=leftmost_column
cpp_indent_access_specifiers=true
cpp_indent_namespace_contents=false
cpp_indent_preserve_comments=false
cpp_new_line_before_open_brace_namespace=ignore
cpp_new_line_before_open_brace_type=ignore
cpp_new_line_before_open_brace_function=ignore
cpp_new_line_before_open_brace_block=ignore
cpp_new_line_before_open_brace_lambda=ignore
cpp_new_line_scope_braces_on_separate_lines=false
cpp_new_line_close_brace_same_line_empty_type=false
cpp_new_line_close_brace_same_line_empty_function=false
cpp_new_line_before_catch=true
cpp_new_line_before_else=false
cpp_new_line_before_while_in_do_while=false
cpp_space_before_function_open_parenthesis=remove
cpp_space_within_parameter_list_parentheses=false
cpp_space_between_empty_parameter_list_parentheses=false
cpp_space_after_keywords_in_control_flow_statements=true
cpp_space_within_control_flow_statement_parentheses=false
cpp_space_before_lambda_open_parenthesis=false
cpp_space_within_cast_parentheses=false
cpp_space_after_cast_close_parenthesis=false
cpp_space_within_expression_parentheses=false
cpp_space_before_block_open_brace=true
cpp_space_between_empty_braces=false
cpp_space_before_initializer_list_open_brace=false
cpp_space_within_initializer_list_braces=true
cpp_space_preserve_in_initializer_list=true
cpp_space_before_open_square_bracket=false
cpp_space_within_square_brackets=false
cpp_space_before_empty_square_brackets=false
cpp_space_between_empty_square_brackets=false
cpp_space_group_square_brackets=true
cpp_space_within_lambda_brackets=false
cpp_space_between_empty_lambda_brackets=false
cpp_space_before_comma=false
cpp_space_after_comma=true
cpp_space_remove_around_member_operators=true
cpp_space_before_inheritance_colon=true
cpp_space_before_constructor_colon=true
cpp_space_remove_before_semicolon=true
cpp_space_after_semicolon=false
cpp_space_remove_around_unary_operator=true
cpp_space_around_binary_operator=insert
cpp_space_around_assignment_operator=insert
cpp_space_pointer_reference_alignment=left
cpp_space_around_ternary_operator=insert
cpp_wrap_preserve_blocks=one_liners
"#;
