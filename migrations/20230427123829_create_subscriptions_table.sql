-- Create subscriptions table
CREATE TABLE subscriptions(
  id uuid PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  subscribed_at timestamptz NOT NULL
);

