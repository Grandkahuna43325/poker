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
-- Name: pictures; Type: TABLE; Schema: public; Owner: grandkahuna43325
--

CREATE TABLE public.pictures (
    id integer NOT NULL,
    item_id integer NOT NULL,
    picture_data character varying NOT NULL
);


ALTER TABLE public.pictures OWNER TO grandkahuna43325;

--
-- Name: pictures_id_seq; Type: SEQUENCE; Schema: public; Owner: grandkahuna43325
--

CREATE SEQUENCE public.pictures_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.pictures_id_seq OWNER TO grandkahuna43325;

--
-- Name: pictures_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: grandkahuna43325
--

ALTER SEQUENCE public.pictures_id_seq OWNED BY public.pictures.id;


--
-- Name: posts; Type: TABLE; Schema: public; Owner: grandkahuna43325
--

CREATE TABLE public.posts (
    id integer NOT NULL,
    name character varying NOT NULL,
    main_picture character varying NOT NULL,
    description character varying NOT NULL,
    date date DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.posts OWNER TO grandkahuna43325;

--
-- Name: posts_id_seq; Type: SEQUENCE; Schema: public; Owner: grandkahuna43325
--

CREATE SEQUENCE public.posts_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.posts_id_seq OWNER TO grandkahuna43325;

--
-- Name: posts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: grandkahuna43325
--

ALTER SEQUENCE public.posts_id_seq OWNED BY public.posts.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: grandkahuna43325
--

CREATE TABLE public.users (
    id integer NOT NULL,
    username text NOT NULL,
    hashed_password text NOT NULL
);


ALTER TABLE public.users OWNER TO grandkahuna43325;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: grandkahuna43325
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.users_id_seq OWNER TO grandkahuna43325;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: grandkahuna43325
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: pictures id; Type: DEFAULT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.pictures ALTER COLUMN id SET DEFAULT nextval('public.pictures_id_seq'::regclass);


--
-- Name: posts id; Type: DEFAULT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.posts ALTER COLUMN id SET DEFAULT nextval('public.posts_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: grandkahuna43325
--

COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
00000000000000	2023-12-26 11:50:46.105768
20231009194722	2023-12-26 11:50:46.108049
20231220153751	2023-12-26 12:20:46.882424
\.


--
-- Data for Name: pictures; Type: TABLE DATA; Schema: public; Owner: grandkahuna43325
--

COPY public.pictures (id, item_id, picture_data) FROM stdin;
162	15	image/jpeg
163	15	image/jpeg
164	15	image/jpeg
165	15	image/jpeg
166	15	image/jpeg
22	2	image/jpeg
23	2	image/jpeg
24	2	image/jpeg
25	2	image/jpeg
26	2	image/jpeg
27	2	image/jpeg
28	2	image/jpeg
29	2	image/jpeg
30	2	image/jpeg
31	2	image/jpeg
32	2	image/jpeg
33	2	image/jpeg
34	2	image/jpeg
35	2	video/mp4
76	7	image/jpeg
77	7	image/jpeg
78	7	image/jpeg
79	7	image/jpeg
80	7	image/jpeg
81	7	image/jpeg
82	7	image/jpeg
83	7	image/jpeg
84	7	image/jpeg
85	8	image/jpeg
86	8	image/jpeg
87	8	image/jpeg
88	8	image/jpeg
89	8	image/jpeg
111	11	image/jpeg
112	11	image/jpeg
113	11	image/jpeg
114	11	image/jpeg
115	11	image/jpeg
116	11	image/jpeg
117	11	image/jpeg
118	11	image/jpeg
128	13	image/jpeg
129	13	image/jpeg
130	13	image/jpeg
131	13	image/jpeg
132	13	image/jpeg
133	13	image/jpeg
134	13	image/jpeg
135	13	image/jpeg
136	13	image/jpeg
137	13	image/jpeg
138	13	image/jpeg
139	13	image/jpeg
140	13	image/jpeg
141	14	image/jpeg
142	14	image/jpeg
143	14	image/jpeg
144	14	image/jpeg
145	14	image/jpeg
146	14	image/jpeg
147	14	image/jpeg
148	14	image/jpeg
149	14	image/jpeg
150	14	video/mp4
\.


--
-- Data for Name: posts; Type: TABLE DATA; Schema: public; Owner: grandkahuna43325
--

COPY public.posts (id, name, main_picture, description, date) FROM stdin;
2	tytuł	image/jpeg	opis	1223-11-15
7	tytuł	image/jpeg	Długi opis	1664-12-15
8	tytuł	image/jpeg	opis	2006-11-12
11	Tytuł	image/jpeg	opis	2006-11-12
13	tytuł	image/jpeg	opis	2255-11-12
14	tytuł	image/jpeg	Długi opis	2008-11-12
15	tytuł	image/jpeg	opis	0012-12-12
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: grandkahuna43325
--

COPY public.users (id, username, hashed_password) FROM stdin;
1	Grandkahuna43325	$argon2id$v=19$m=307200,t=1,p=4$l3v4s506a/YL8DXkg62NGw$cucZSJuf4GC5hiRYOa9MhHQ9dpOJ1x1321t8EwJjY58
3	u	$argon2id$v=19$m=307200,t=1,p=4$tiqt4WLXpdDHtfz1R2JORg$2Huq/fjXfhwS+mnbmo+0KtJ236hNV5vkodtWeXSOyFI
\.


--
-- Name: pictures_id_seq; Type: SEQUENCE SET; Schema: public; Owner: grandkahuna43325
--

SELECT pg_catalog.setval('public.pictures_id_seq', 171, true);


--
-- Name: posts_id_seq; Type: SEQUENCE SET; Schema: public; Owner: grandkahuna43325
--

SELECT pg_catalog.setval('public.posts_id_seq', 16, true);


--
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: grandkahuna43325
--

SELECT pg_catalog.setval('public.users_id_seq', 8, true);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: pictures pictures_pkey; Type: CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.pictures
    ADD CONSTRAINT pictures_pkey PRIMARY KEY (id);


--
-- Name: posts posts_pkey; Type: CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.posts
    ADD CONSTRAINT posts_pkey PRIMARY KEY (id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: users users_username_key; Type: CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_username_key UNIQUE (username);


--
-- Name: pictures pictures_item_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.pictures
    ADD CONSTRAINT pictures_item_id_fkey FOREIGN KEY (item_id) REFERENCES public.posts(id);


--
-- PostgreSQL database dump complete
--

