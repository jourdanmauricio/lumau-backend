INSERT INTO admin_users (name, url, front_deploy, email, password, status, role, created_at, updated_at) 
VALUES (:name, :url, :front_deploy, :email, :password, :status, :role, datetime('now', 'localtime'),:updated);
