CREATE OR REPLACE FUNCTION extract_device_type(ua TEXT)
RETURNS TEXT AS $$
BEGIN
    RETURN CASE
        WHEN lower(ua) LIKE '%linux%android%' THEN 'Android'
        WHEN lower(ua) LIKE '%iphone%' THEN 'Iphone'
        WHEN lower(ua) LIKE '%linux%' THEN 'Linux'
        WHEN lower(ua) LIKE '%windows%' THEN 'Windows'
        WHEN lower(ua) LIKE '%mac os%' THEN 'Mac'
        ELSE 'Other'
    END;
END;
$$ LANGUAGE plpgsql IMMUTABLE;
