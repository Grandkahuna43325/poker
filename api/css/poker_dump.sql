--
-- PostgreSQL database dump
--

-- Dumped from database version 15.4
-- Dumped by pg_dump version 15.4

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: grandkahuna43325
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO grandkahuna43325;

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: grandkahuna43325
--

CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO grandkahuna43325;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: grandkahuna43325
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO grandkahuna43325;

--
-- Name: admin; Type: TABLE; Schema: public; Owner: grandkahuna43325
--

CREATE TABLE public.admin (
    id integer NOT NULL,
    username character varying(25) NOT NULL,
    password character varying(255) NOT NULL
);


ALTER TABLE public.admin OWNER TO grandkahuna43325;

--
-- Name: admin_id_seq; Type: SEQUENCE; Schema: public; Owner: grandkahuna43325
--

CREATE SEQUENCE public.admin_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.admin_id_seq OWNER TO grandkahuna43325;

--
-- Name: admin_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: grandkahuna43325
--

ALTER SEQUENCE public.admin_id_seq OWNED BY public.admin.id;


--
-- Name: player; Type: TABLE; Schema: public; Owner: grandkahuna43325
--

CREATE TABLE public.player (
    id integer NOT NULL,
    name character varying(25) NOT NULL,
    score integer NOT NULL,
    image_url character varying(1000) NOT NULL
);


ALTER TABLE public.player OWNER TO grandkahuna43325;

--
-- Name: player_id_seq; Type: SEQUENCE; Schema: public; Owner: grandkahuna43325
--

CREATE SEQUENCE public.player_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.player_id_seq OWNER TO grandkahuna43325;

--
-- Name: player_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: grandkahuna43325
--

ALTER SEQUENCE public.player_id_seq OWNED BY public.player.id;


--
-- Name: soul; Type: TABLE; Schema: public; Owner: grandkahuna43325
--

CREATE TABLE public.soul (
    id integer NOT NULL,
    owner integer NOT NULL,
    name character varying(25) NOT NULL
);


ALTER TABLE public.soul OWNER TO grandkahuna43325;

--
-- Name: soul_id_seq; Type: SEQUENCE; Schema: public; Owner: grandkahuna43325
--

CREATE SEQUENCE public.soul_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.soul_id_seq OWNER TO grandkahuna43325;

--
-- Name: soul_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: grandkahuna43325
--

ALTER SEQUENCE public.soul_id_seq OWNED BY public.soul.id;


--
-- Name: admin id; Type: DEFAULT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.admin ALTER COLUMN id SET DEFAULT nextval('public.admin_id_seq'::regclass);


--
-- Name: player id; Type: DEFAULT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.player ALTER COLUMN id SET DEFAULT nextval('public.player_id_seq'::regclass);


--
-- Name: soul id; Type: DEFAULT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.soul ALTER COLUMN id SET DEFAULT nextval('public.soul_id_seq'::regclass);


--
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: grandkahuna43325
--

COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
00000000000000	2024-01-18 20:02:51.909033
20240118200317	2024-01-22 20:52:06.38985
\.


--
-- Data for Name: admin; Type: TABLE DATA; Schema: public; Owner: grandkahuna43325
--

COPY public.admin (id, username, password) FROM stdin;
1	Grandkahuna43325	$argon2id$v=19$m=307200,t=1,p=4$l3v4s506a/YL8DXkg62NGw$cucZSJuf4GC5hiRYOa9MhHQ9dpOJ1x1321t8EwJjY58
2	u	$argon2id$v=19$m=307200,t=1,p=4$h89+6TrGR6SrHh1nXcq8Kg$NtGO3MFPBp5MuoyA/xO0EePF/cXbsQAb8qPnRX3pRz8
\.


--
-- Data for Name: player; Type: TABLE DATA; Schema: public; Owner: grandkahuna43325
--

COPY public.player (id, name, score, image_url) FROM stdin;
3	Nigor	-613	https://scontent-fra5-1.xx.fbcdn.net/v/t39.30808-1/343928860_966195758030693_7662706195012964935_n.jpg?stp=dst-jpg_s320x320&_nc_cat=110&ccb=1-7&_nc_sid=5740b7&_nc_ohc=ghODcU9V4jcAX9Ko5Rp&_nc_ht=scontent-fra5-1.xx&oh=00_AfCnTdTwQQDvfWH5Ij-skVt8OgDPStkVZI_Znldr2GOOjw&oe=65B47EFE
1	Franek	6211	https://scontent-ham3-1.xx.fbcdn.net/v/t1.6435-1/116429134_613241079586644_1275122240777393675_n.jpg?stp=dst-jpg_p320x320&_nc_cat=108&ccb=1-7&_nc_sid=2b6aad&_nc_ohc=t-Jj6CsmnjcAX9-xtXH&_nc_ht=scontent-ham3-1.xx&oh=00_AfDb0kkQ3OTd3SlzpQrFneEJpRB0xxNHSKWP51TWiCmsew&oe=65D64202
2	u	1383	https://scontent-fra3-2.xx.fbcdn.net/v/t39.30808-6/393402094_1030790888113381_2763569305068059617_n.jpg?stp=dst-jpg_p843x403&_nc_cat=111&ccb=1-7&_nc_sid=efb6e6&_nc_ohc=UjTL8Vrq4GMAX-lELvi&_nc_ht=scontent-fra3-2.xx&oh=00_AfDJ6mUiu8zNPgnezpS0e0ruzRc0_EA1wUoB1IoQVDxGWw&oe=65B4D070
\.


--
-- Data for Name: soul; Type: TABLE DATA; Schema: public; Owner: grandkahuna43325
--

COPY public.soul (id, owner, name) FROM stdin;
1	1	Franek
2	2	u
3	3	Nigor
\.


--
-- Name: admin_id_seq; Type: SEQUENCE SET; Schema: public; Owner: grandkahuna43325
--

SELECT pg_catalog.setval('public.admin_id_seq', 2, true);


--
-- Name: player_id_seq; Type: SEQUENCE SET; Schema: public; Owner: grandkahuna43325
--

SELECT pg_catalog.setval('public.player_id_seq', 3, true);


--
-- Name: soul_id_seq; Type: SEQUENCE SET; Schema: public; Owner: grandkahuna43325
--

SELECT pg_catalog.setval('public.soul_id_seq', 3, true);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: admin admin_pkey; Type: CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.admin
    ADD CONSTRAINT admin_pkey PRIMARY KEY (id);


--
-- Name: player player_name_key; Type: CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.player
    ADD CONSTRAINT player_name_key UNIQUE (name);


--
-- Name: player player_pkey; Type: CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.player
    ADD CONSTRAINT player_pkey PRIMARY KEY (id);


--
-- Name: soul soul_pkey; Type: CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.soul
    ADD CONSTRAINT soul_pkey PRIMARY KEY (id);


--
-- Name: soul soul_name_fkey; Type: FK CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.soul
    ADD CONSTRAINT soul_name_fkey FOREIGN KEY (name) REFERENCES public.player(name);


--
-- Name: soul soul_owner_fkey; Type: FK CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.soul
    ADD CONSTRAINT soul_owner_fkey FOREIGN KEY (owner) REFERENCES public.player(id);


--
-- PostgreSQL database dump complete
--

