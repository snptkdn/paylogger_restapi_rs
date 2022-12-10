-- Add migration script here
ALTER TABLE category ADD PRIMARY KEY (id);
ALTER TABLE category CHANGE id id INT AUTO_INCREMENT;