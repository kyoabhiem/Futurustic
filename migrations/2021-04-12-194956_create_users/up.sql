CREATE TABLE users
(
    id         VARCHAR(36)  NOT NULL PRIMARY KEY,
    name       varchar(255) not null,
    email      varchar(255) not null unique,
    password   varchar(255) not null,
    created_by varchar(36)  null,
    created_at timestamp    not null default now(),
    updated_by varchar(36)  null,
    updated_at timestamp    not null default now(),
    deleted_by varchar(36)  null,
    deleted_at timestamp    null
);
INSERT INTO `users` (`id`, `name`, `email`, `password`, `created_by`, `created_at`, `updated_by`, `updated_at`, `deleted_by`, `deleted_at`) VALUES
('7e06a184-14df-40de-9482-41816ffef33b', 'user1', 'user1@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('0e0cca55-7419-4506-b6c9-01d18a0b7919', 'user2', 'user2@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('51ac0b2d-e502-4628-89ef-4ac9094dcbc8', 'user3', 'user3@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('fffe9dbc-783b-4d2c-b3b0-54087ac27d33', 'user4', 'user4@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('e81d88bf-440c-4d6a-8bed-24d654ec1dfe', 'user5', 'user5@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('d5ed67a1-59a8-4d74-8894-09e9db148a79', 'user6', 'user6@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('9d4b1b5b-990c-423d-a472-1b03a61afe21', 'user7', 'user7@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('50765a53-9880-4c26-b7ea-e42a0d0e5769', 'user8', 'user8@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('5a963ccc-8ff7-4dd3-a537-3253777903e4', 'user9', 'user9@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('ad15bd98-97bf-46f6-810b-dfc439a10d5f', 'user10', 'user10@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('201529e6-a889-4024-a7e7-79d0226b8861', 'user11', 'user11@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('22521d30-05b9-42e1-abd5-e52b5a9ddbf8', 'user12', 'user12@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('80cd8e78-d311-4325-9d9a-2e28ff6b6a1a', 'user13', 'user13@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('48083baf-2e3d-4a76-bab0-df93684da3c9', 'user14', 'user14@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('65b29c7b-3910-4b83-9360-094eb9fe42af', 'user15', 'user15@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('566ebffd-b6b0-4454-a37e-0bcdc29eaa6a', 'user16', 'user16@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('e24d9382-e258-4f32-bf5b-e519c7316f41', 'user17', 'user17@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('3eb9dd95-b569-49c7-8b73-087c3bafac7f', 'user18', 'user18@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('7a22c1fc-80a0-45b9-9db4-30e281db7d92', 'user19', 'user19@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('ae98c2ec-359a-4d8e-9ae4-d22031e24be4', 'user20', 'user20@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null),
('6e34bfd0-0467-4cb5-9d8b-d8a08aeef03f', 'user21', 'user21@example.com', 'secret', null, CURRENT_TIMESTAMP, null, CURRENT_TIMESTAMP, null, null)
;