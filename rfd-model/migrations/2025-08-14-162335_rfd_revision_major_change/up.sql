-- Adding the column in three steps to ensure all rows have a default.
ALTER TABLE rfd_revision ADD COLUMN major_change BOOLEAN;
UPDATE rfd_revision SET major_change = TRUE;
ALTER TABLE rfd_revision ALTER COLUMN major_change SET NOT NULL;
