
CREATE TYPE priority_type AS ENUM (
    'low',
    'medium',
    'high'
);

CREATE TYPE status_type as ENUM (
    'open',
    'closed'
);

CREATE TABLE IF NOT EXISTS todos_list (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id UUID NOT NULL,
    FOREIGN KEY(user_id)
        REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS todos (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    list_id UUID NOT NULL,
    FOREIGN KEY(list_id)
        REFERENCES todos_list(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    priority priority_type NOT NULL DEFAULT 'medium',
    status status_type NOT NULL DEFAULT 'open',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
