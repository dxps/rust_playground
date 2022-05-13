
-- DEV ONLY - Comment out for keeping db between restarts.
DROP DATABASE IF EXISTS todo_app;
DROP USER IF EXISTS todo_app;

-- DEV ONLY - For quick iterations.
CREATE USER todo_app PASSWORD 'todo_app';
CREATE DATABASE todo_app owner todo_app ENCODING = 'UTF-8';
