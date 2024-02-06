-- Your SQL goes here
CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  email TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);

CREATE TABLE patients (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    date_of_birth DATE NOT NULL,
    contact_number VARCHAR(20) NOT NULL,
    email VARCHAR(100),
    address VARCHAR(200),
    attendant_id INTEGER NOT NULL REFERENCES users(id)
);

CREATE TABLE attendance_records (
   id SERIAL PRIMARY KEY,
   patient_id INTEGER NOT NULL REFERENCES patients(id),
   attendant_id INTEGER NOT NULL REFERENCES users(id),
   attendance_date DATE NOT NULL,
   canceled BOOLEAN DEFAULT FALSE,
   notes TEXT
);

CREATE TABLE access_tokens (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    token VARCHAR(500) NOT NULL,
    expiry_date TIMESTAMP NOT NULL
);