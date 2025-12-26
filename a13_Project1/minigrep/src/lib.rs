pub fn search<'a>(query:&str,contents:&'a str)-> Vec<&'a str>{
    let mut results = Vec::new();
for lines in contents.lines(){//contents.lines returns lines in the content
    if lines.contains(query){//here lines.contains searches for the query in the line
        results.push(lines);//pushes it into vector if it matches the query
    }
}
results
}
pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)-> Vec<&'a str>{
    let mut results=Vec::new();
    let query=query.to_lowercase();
    for lines in contents.lines(){
        if lines.to_lowercase().contains(&query){
            results.push(lines);
        }
    }
    results
}




#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn one_result(){
        let query="duct";
        let contents="\
Rust:
safe, fast, productive.
Pick, three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query,contents));
    }
    #[test]
    fn two_result(){
        let query="rUST";
        let content= "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."],search_case_insensitive(query,content));

    }
}
