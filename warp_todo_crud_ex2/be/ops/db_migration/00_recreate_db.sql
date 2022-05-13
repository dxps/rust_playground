
-- DEV ONLY - Comment out for keeping db between restarts.
DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_user;

-- DEV ONLY - For quick iterations.
CREATE USER test PASSWORD 'test';
CREATE DATABASE test owner test ENCODING = 'UTF-8';
