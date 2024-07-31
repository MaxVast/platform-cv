-- Your SQL goes here
DO $$
DECLARE
    Id_Syneidolab UUID;
BEGIN
    SELECT id INTO Id_Syneidolab
    FROM company
    WHERE name = 'SyneidoLAB';

    INSERT INTO users (username, email, password, role, company_id, login_session)
    VALUES ('superadmin', 'mvast@syneidolab.com', '$2y$10$2FbcAoMDLmpFUz/phu0Pa.0Yhi6B9VU1ag9uRSgcW2lZhlh0U6M2C', 'superadmin', Id_Syneidolab, null);
END $$;