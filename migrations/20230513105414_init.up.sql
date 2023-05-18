CREATE TABLE `user` (
    user_id binary(16) PRIMARY KEY,
    user_id_text varchar(36) generated always as
    (insert(
        insert(
        insert(
            insert(hex(user_id),9,0,'-'),
            14,0,'-'),
        19,0,'-'),
        24,0,'-')
    ) virtual,
    user_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    hash_pass VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW() ON UPDATE NOW()
);

CREATE TABLE `post` (
    id CHAR(36) NOT NULL PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID())),
    title VARCHAR(255) NOT NULL UNIQUE,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);