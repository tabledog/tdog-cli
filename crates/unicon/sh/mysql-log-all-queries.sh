# Log all queries to understand:
# - What the Rust MySQL API is doing.
# - Syntax errors.
# @see https://stackoverflow.com/questions/22609257/how-do-i-enable-the-mysql-slow-query-log
mysql -h 127.0.0.1 -pmy-secret-pw -u root -e "set global slow_query_log = 'ON';  set global long_query_time = 0; set global slow_query_log_file ='/del/mysql-slow-query.log'; show variables like '%slow%';";