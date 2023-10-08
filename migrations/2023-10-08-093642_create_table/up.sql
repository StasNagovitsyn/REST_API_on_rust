CREATE TABLE IF NOT EXISTS public.authors
(
    authors_id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 14 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    name text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT authors_pkey PRIMARY KEY (authors_id),
    CONSTRAINT name_ UNIQUE (name)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.authors
    OWNER to postgres;


CREATE TABLE IF NOT EXISTS public.books
(
    books_id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 5 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    title text COLLATE pg_catalog."default" NOT NULL,
    fk_authors_id integer NOT NULL,
    CONSTRAINT books_pkey PRIMARY KEY (books_id),
    CONSTRAINT books_fk_authors_id_fkey FOREIGN KEY (fk_authors_id)
        REFERENCES public.authors (authors_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.books
    OWNER to postgres;