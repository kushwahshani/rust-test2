CREATE TABLE IF NOT EXISTS student(
    id INT PRIMARY KEY,
    name VARCHAR(20),
    age NOT NULL
);

INSERT INTO student (id, name, age) VALUES (1, 'shani', 20);
INSERT INTO student (id , name, age) VALUES(2, 'kushwah', 21);



wrangler d1 execute DB --local --command "CREATE TABLE IF NOT EXISTS student ( id INT PRIMARY KEY,name VARCHAR(20),email VARCHAR(20),age NOT NULL);"

wrangler d1 execute DB --local --command "INSERT INTO student (id, name, age) VALUES (1, 'shani', 20);"C
wrangler d1 execute DB --local --command "INSERT INTO student (id, name , age) VALUES(3, 'shani11', 21)"
wrangler d1 execute DB --local --command "DELETE FROM shops WHERE shop = 'ac-dev-25.myshopify.com';"

CREATE TABLE IF NOT EXISTS shops(
    shop VARCHAR(100) PRIMARY KEY,
    auth_token VARCHAR(100)
    installation BOOLEAN
)

wrangler d1 execute DB --local --command "CREATE TABLE IF NOT EXISTS shops ( shop VARCHAR(100) PRIMARY KEY,auth_token VARCHAR(100),installation BOOLEAN);"
 wrangler d1 execute DB --local --command "DROP TABLE IF EXISTS shops;"  
--  online create a table in cloudflare
 wrangler d1 execute DB --remote --command "CREATE TABLE IF NOT EXISTS shops ( shop VARCHAR(100) PRIMARY KEY,auth_token VARCHAR(100),installation BOOLEAN);"