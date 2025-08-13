INSERT INTO users (uid, name) VALUES ($1, $2)
    ON CONFLICT (uid)
        DO NOTHING RETURNING uid
