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
-- Name: logs; Type: TABLE; Schema: public; Owner: grandkahuna43325
--

CREATE TABLE public.logs (
    id integer NOT NULL,
    date timestamp without time zone NOT NULL,
    log text NOT NULL,
    admin_id integer NOT NULL
);


ALTER TABLE public.logs OWNER TO grandkahuna43325;

--
-- Name: logs_id_seq; Type: SEQUENCE; Schema: public; Owner: grandkahuna43325
--

CREATE SEQUENCE public.logs_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.logs_id_seq OWNER TO grandkahuna43325;

--
-- Name: logs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: grandkahuna43325
--

ALTER SEQUENCE public.logs_id_seq OWNED BY public.logs.id;


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
-- Name: logs id; Type: DEFAULT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.logs ALTER COLUMN id SET DEFAULT nextval('public.logs_id_seq'::regclass);


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
20240118200317	2024-01-23 20:01:55.202125
20240124181403	2024-01-24 18:15:42.905605
\.


--
-- Data for Name: admin; Type: TABLE DATA; Schema: public; Owner: grandkahuna43325
--

COPY public.admin (id, username, password) FROM stdin;
1	Grandkahuna43325	$argon2id$v=19$m=307200,t=1,p=4$l3v4s506a/YL8DXkg62NGw$cucZSJuf4GC5hiRYOa9MhHQ9dpOJ1x1321t8EwJjY58
3	Nowosz	$argon2id$v=19$m=307200,t=1,p=4$mFDFkdiv392uwcSq4Cpq5g$auKj0n2TffKYNcUhMlv1BOkmSKDtUgY3WAJvcUlC+GM
4	admin	$argon2id$v=19$m=307200,t=1,p=4$BBsSOZCZFWkCQN71h6Kd5Q$E1GHpO2LCn6hL3iJArHul0tTNbyR9GnTj6MBSV2qKJk
\.


--
-- Data for Name: logs; Type: TABLE DATA; Schema: public; Owner: grandkahuna43325
--

COPY public.logs (id, date, log, admin_id) FROM stdin;
2	2024-01-24 18:57:53.151075	player id: 16 name: test 3 folded: true score: -7\nplayer id: 15 name: test 2 folded: false score: 51\nplayer id: 13 name: test folded: false score: -22\nplayer id: 17 name: test 4 folded: false score: -22	3
3	2024-01-24 18:59:18.766004	player id: 15 name: test 2 folded: true score: -11\nplayer id: 13 name: test folded: true score: -111\nplayer id: 16 name: test 3 folded: true score: -116\nplayer id: 17 name: test 4 folded: false score: 237	3
4	2024-01-25 09:42:26.133861	player id: 3 name: Nowosz folded: true score: -1\nplayer id: 12 name: Siluk folded: true score: -5\nplayer id: 6 name: Antek folded: true score: -5\nplayer id: 11 name: Gałecki folded: true score: -5\nplayer id: 5 name: Nigor folded: false score: 36\nplayer id: 7 name: Jacek folded: false score: -20	3
5	2024-01-25 09:43:29.95809	player id: 3 name: Nowosz folded: true score: -1\nplayer id: 5 name: Nigor folded: true score: -1\nplayer id: 12 name: Siluk folded: true score: -1\nplayer id: 7 name: Jacek folded: true score: -1\nplayer id: 6 name: Antek folded: false score: 21\nplayer id: 11 name: Gałecki folded: false score: -21	3
6	2024-01-25 09:44:39.375686	player id: 3 name: Nowosz folded: true score: -1\nplayer id: 5 name: Nigor folded: true score: -1\nplayer id: 7 name: Jacek folded: true score: -1\nplayer id: 6 name: Antek folded: true score: -1\nplayer id: 11 name: Gałecki folded: false score: 30\nplayer id: 12 name: Siluk folded: false score: -30	3
7	2024-01-25 09:45:54.679421	player id: 11 name: Gałecki folded: true score: -1\nplayer id: 7 name: Jacek folded: true score: -6\nplayer id: 6 name: Antek folded: true score: -6\nplayer id: 3 name: Nowosz folded: true score: -10\nplayer id: 12 name: Siluk folded: false score: 34\nplayer id: 5 name: Nigor folded: false score: -15	3
8	2024-01-25 09:47:13.083262	player id: 5 name: Nigor folded: true score: -1\nplayer id: 6 name: Antek folded: true score: -1\nplayer id: 12 name: Siluk folded: false score: 13\nplayer id: 3 name: Nowosz folded: false score: -5\nplayer id: 11 name: Gałecki folded: false score: -5\nplayer id: 7 name: Jacek folded: false score: -5	3
9	2024-01-25 09:48:26.67719	player id: 12 name: Siluk folded: true score: -1\nplayer id: 11 name: Gałecki folded: true score: -1\nplayer id: 6 name: Antek folded: true score: -15\nplayer id: 7 name: Jacek folded: false score: 45\nplayer id: 3 name: Nowosz folded: false score: -15\nplayer id: 5 name: Nigor folded: false score: -15	3
10	2024-01-25 09:49:40.043425	player id: 11 name: Gałecki folded: true score: -1\nplayer id: 7 name: Jacek folded: true score: -1\nplayer id: 5 name: Nigor folded: true score: -1\nplayer id: 12 name: Siluk folded: true score: -1\nplayer id: 6 name: Antek folded: true score: -1\nplayer id: 3 name: Nowosz folded: true score: -2\nplayer id: 2 name: Odziom folded: false score: 7	3
11	2024-01-25 09:50:40.354666	player id: 6 name: Antek folded: true score: -1\nplayer id: 5 name: Nigor folded: true score: -1\nplayer id: 2 name: Odziom folded: true score: -1\nplayer id: 12 name: Siluk folded: true score: -1\nplayer id: 7 name: Jacek folded: true score: -11\nplayer id: 3 name: Nowosz folded: false score: 29\nplayer id: 11 name: Gałecki folded: false score: -20	3
12	2024-01-25 09:56:23.172155	player id: 16 name: test 3 folded: true score: -1\nplayer id: 13 name: test folded: true score: -1\nplayer id: 17 name: test 4 folded: true score: -11\nplayer id: 14 name: test2 folded: false score: 13	3
13	2024-01-25 09:56:57.77198	player id: 16 name: test 3 folded: true score: -1\nplayer id: 13 name: test folded: true score: -1\nplayer id: 14 name: test2 folded: false score: 6\nplayer id: 17 name: test 4 folded: false score: -7	3
14	2024-01-25 09:58:06.03044	player id: 16 name: test 3 folded: true score: -1\nplayer id: 13 name: test folded: true score: -2\nplayer id: 14 name: test2 folded: false score: 3\nplayer id: 17 name: test 4 folded: false score: -2	3
15	2024-01-25 09:59:56.593458	player id: 13 name: test folded: true score: -1\nplayer id: 17 name: test 4 folded: true score: -1\nplayer id: 15 name: test 2 folded: false score: 18\nplayer id: 16 name: test 3 folded: false score: -8	3
16	2024-01-25 10:00:58.191039	player id: 16 name: test 3 folded: true score: -1\nplayer id: 14 name: test2 folded: true score: -1\nplayer id: 13 name: test folded: false score: 14\nplayer id: 17 name: test 4 folded: false score: -6	3
17	2024-01-25 10:06:17.2013	player id: 13 name: test folded: false score: 3\nplayer id: 15 name: test 2 folded: false score: -1\nplayer id: 16 name: test 3 folded: false score: -1\nplayer id: 17 name: test 4 folded: false score: -1	3
18	2024-01-25 10:11:46.745232	player id: 17 name: test 4 folded: true score: -1\nplayer id: 16 name: test 3 folded: true score: -2\nplayer id: 15 name: test 2 folded: true score: -2\nplayer id: 13 name: test folded: false score: 5	3
19	2024-01-25 10:12:01.663215	player id: 17 name: test 4 folded: true score: -1\nplayer id: 16 name: test 3 folded: true score: -1\nplayer id: 13 name: test folded: false score: 2\nplayer id: 15 name: test 2 folded: false score: -3	3
20	2024-01-25 10:13:44.390287	player id: 17 name: test 4 folded: true score: -1\nplayer id: 16 name: test 3 folded: true score: -1\nplayer id: 13 name: test folded: false score: 5\nplayer id: 15 name: test 2 folded: false score: -3	3
21	2024-01-25 10:18:18.274655	player id: 17 name: test 4 folded: true score: -1\nplayer id: 16 name: test 3 folded: true score: -1\nplayer id: 13 name: test folded: false score: 4\nplayer id: 15 name: test 2 folded: false score: -2	3
22	2024-01-25 10:19:20.520944	player id: 17 name: test 4 folded: true score: -1\nplayer id: 16 name: test 3 folded: true score: -1\nplayer id: 13 name: test folded: false score: 3\nplayer id: 15 name: test 2 folded: false score: -1	3
23	2024-01-25 10:19:46.042948	player id: 17 name: test 4 folded: true score: -1\nplayer id: 16 name: test 3 folded: true score: -1\nplayer id: 13 name: test folded: false score: 3\nplayer id: 15 name: test 2 folded: false score: -1	3
24	2024-01-25 10:20:01.999929	player id: 17 name: test 4 folded: true score: -1\nplayer id: 16 name: test 3 folded: true score: -1\nplayer id: 13 name: test folded: false score: 5\nplayer id: 15 name: test 2 folded: false score: -3	3
25	2024-01-25 10:21:54.826302	player id: 17 name: test 4 folded: true score: -1\nplayer id: 16 name: test 3 folded: true score: -1\nplayer id: 13 name: test folded: false score: 13\nplayer id: 15 name: test 2 folded: false score: -11	3
26	2024-01-25 10:22:04.893098	player id: 17 name: test 4 folded: true score: -1\nplayer id: 16 name: test 3 folded: true score: -1\nplayer id: 13 name: test folded: false score: 14\nplayer id: 15 name: test 2 folded: false score: -12	3
27	2024-01-25 10:24:03.180811	player id: 17 name: test 4 folded: true score: -6\nplayer id: 16 name: test 3 folded: true score: -6\nplayer id: 15 name: test 2 folded: true score: -7\nplayer id: 13 name: test folded: false score: 19	3
28	2024-02-19 20:52:00.312889	player id: 23 name: temp 1 folded: true score: -2\nplayer id: 24 name: temp 2 folded: true score: -3\nplayer id: 25 name: temp 3 folded: true score: -3\nplayer id: 27 name: temp 5 folded: false score: 14\nplayer id: 26 name: temp 4 folded: false score: -3	4
\.


--
-- Data for Name: player; Type: TABLE DATA; Schema: public; Owner: grandkahuna43325
--

COPY public.player (id, name, score, image_url) FROM stdin;
3	Nowoszow	-512	https://scontent.xx.fbcdn.net/v/t1.15752-9/427003807_919609092906668_5696564173303203004_n.png?stp=dst-png_s206x206&_nc_cat=105&ccb=1-7&_nc_sid=510075&_nc_ohc=78zLzfkO_qAAX-8F0_J&_nc_ad=z-m&_nc_cid=0&_nc_ht=scontent.xx&oh=03_AdT8mCac2WKoiYwrz2bTwCU64pIVcJpY86dP-muaSvVOwA&oe=65F59BDD
18	Wieczorek	811	https://scontent.xx.fbcdn.net/v/t1.15752-9/426836695_7182672001847116_4748092284608140116_n.png?stp=dst-png_s206x206&_nc_cat=103&ccb=1-7&_nc_sid=510075&_nc_ohc=eMRbNr2uOe0AX-V0uHe&_nc_ad=z-m&_nc_cid=0&_nc_ht=scontent.xx&oh=03_AdR_c8s9E1sRWFsMyrZptaLZcc4RBdKaN1w4ztCGu7m4-g&oe=65F59C5E
12	Siluk	1076	https://scontent.xx.fbcdn.net/v/t1.15752-9/417285041_1090657322358646_8673350628865477220_n.jpg?_nc_cat=110&ccb=1-7&_nc_sid=510075&_nc_ohc=_x2EydWYxFUAX_-D5H-&_nc_ad=z-m&_nc_cid=0&_nc_ht=scontent.xx&oh=03_AdST2uEdmEVIpEXb5nroeymojTW_7MByc9vRGfC4nrrnQA&oe=65D85261
11	Sebastian Gałecki 	482	https://scontent.xx.fbcdn.net/v/t1.15752-9/426833006_3737160419862304_4287833029990842320_n.png?stp=dst-png_s206x206&_nc_cat=109&ccb=1-7&_nc_sid=510075&_nc_ohc=jcXOhjJjsUkAX8pCWvx&_nc_ad=z-m&_nc_cid=0&_nc_ht=scontent.xx&oh=03_AdSvgf9fkQRylta1LEhMEFiE5HLQ_r5zph6yF3G8B8u1gQ&oe=65F59E1C
2	Odziom	1364	https://scontent-waw1-1.xx.fbcdn.net/v/t39.30808-6/393402094_1030790888113381_2763569305068059617_n.jpg?_nc_cat=111&ccb=1-7&_nc_sid=efb6e6&_nc_ohc=wYqryzUbv3AAX9v0PFy&_nc_ht=scontent-waw1-1.xx&oh=00_AfAvLLgf-RJxioy0ZL6sVL8ktUdHWm_0k3PiB5sOcKg8SQ&oe=65D27A30
21	Klaudia 	526	https://scontent.xx.fbcdn.net/v/t1.15752-9/427990268_2139864803029944_1571250942816024636_n.png?stp=dst-png_p75x225&_nc_cat=111&ccb=1-7&_nc_sid=510075&_nc_ohc=YOJmFIJ5lXAAX9esKWd&_nc_ad=z-m&_nc_cid=0&_nc_ht=scontent.xx&oh=03_AdTdl3T3185Oi5a4hilPF1pdMV_aRqJ7LZaUTXsjNmcylA&oe=65F5C065
9	Filip Gomółka 	1141	https://scontent.xx.fbcdn.net/v/t1.15752-9/428088691_3771562123081119_5504949379872865544_n.png?stp=dst-png_p206x206&_nc_cat=100&ccb=1-7&_nc_sid=510075&_nc_ohc=sw9KgjvvXIYAX_28gpn&_nc_ad=z-m&_nc_cid=0&_nc_ht=scontent.xx&oh=03_AdSNEMi5xVqN9MU4zpe8regGpxK-4AKBwzNEPyH9vdW6QQ&oe=65F594F3
1	Franek	3141	https://scontent.xx.fbcdn.net/v/t1.15752-9/427861574_748143830371965_7469848815357104061_n.png?stp=dst-png_s206x206&_nc_cat=100&ccb=1-7&_nc_sid=510075&_nc_ohc=Z-6OuUpRrAUAX8AzMxl&_nc_ad=z-m&_nc_cid=0&_nc_ht=scontent.xx&oh=03_AdR3DoD7Tc-b34i5KZUT_AQkOFemUcEr-rEx3iObwoE80g&oe=65F5BAC2
20	Bartek	984	https://scontent.xx.fbcdn.net/v/t1.15752-9/426984876_1520103382171074_1094301564713975448_n.png?_nc_cat=106&ccb=1-7&_nc_sid=510075&_nc_ohc=7MXLt4MPKl4AX9zxAWB&_nc_ad=z-m&_nc_cid=0&_nc_ht=scontent.xx&oh=03_AdT0O6uDPl6meuyNriwCy51iuG53fQyYF_T86q2kj1T7xQ&oe=65F5A189
7	Jacek	2750	https://scontent-waw1-1.xx.fbcdn.net/v/t39.30808-6/224607522_1419903485049501_5739431582515728281_n.jpg?_nc_cat=101&ccb=1-7&_nc_sid=efb6e6&_nc_ohc=UbRyaE-Ew2IAX8o5hb9&_nc_ht=scontent-waw1-1.xx&oh=00_AfCdbGDapROJuL25PM9eyekCPWos2PODRq7fKU8qv9A6YA&oe=65D3A3C5
22	Piotr Gomółka 	294	https://scontent.xx.fbcdn.net/v/t1.15752-9/427067485_314489111153994_7236016471787939470_n.png?stp=dst-png_s206x206&_nc_cat=109&ccb=1-7&_nc_sid=510075&_nc_ohc=QSfM6s3g2HwAX-g9dFb&_nc_ad=z-m&_nc_cid=0&_nc_ht=scontent.xx&oh=03_AdRGXsf33Ri8P06Zs96Yy2_NCX9Vv0-Y5VSEgAipnDnkgA&oe=65F592AE
5	Nigor	648	https://scontent.xx.fbcdn.net/v/t1.15752-9/427850610_1327605954447001_8065543151249135144_n.png?stp=dst-png_p160x160&_nc_cat=106&ccb=1-7&_nc_sid=510075&_nc_ohc=uup06ZbI6zgAX-rMLEX&_nc_ad=z-m&_nc_cid=0&_nc_ht=scontent.xx&oh=03_AdTt0W4Zh6i757G8hnfn0JudCYyZTok1siYESW5mgnctUA&oe=65F5AC18
8	Żaba	-468	https://scontent.xx.fbcdn.net/v/t1.15752-9/426984874_1529621584247172_4167296837232901756_n.png?stp=dst-png_s206x206&_nc_cat=101&ccb=1-7&_nc_sid=510075&_nc_ohc=2beEXFd4FPkAX_1a5ze&_nc_ad=z-m&_nc_cid=0&_nc_ht=scontent.xx&oh=03_AdTrON3uqXmTGt26AEz5HTFVk5_IvEhwoKFiRrIUjfgbWA&oe=65F5B823
6	Antoni 	2503	https://scontent.xx.fbcdn.net/v/t1.15752-9/426886705_403636202268248_3162802914494701158_n.png?stp=dst-png_p206x206&_nc_cat=107&ccb=1-7&_nc_sid=510075&_nc_ohc=7ZxSHue7HyQAX_GlsFi&_nc_ad=z-m&_nc_cid=0&_nc_ht=scontent.xx&oh=03_AdQMZ0W7zUncpKfET4KlHzE79ufkeHa77SbJUFkVIPrZOw&oe=65F5CBC0
10	Olek Kot	1837	https://scontent.xx.fbcdn.net/v/t1.15752-9/426828355_2262358707301399_140352334808694653_n.png?_nc_cat=106&ccb=1-7&_nc_sid=510075&_nc_ohc=oK_ru5hxXmcAX9y3E_Z&_nc_ad=z-m&_nc_cid=0&_nc_ht=scontent.xx&oh=03_AdSw07NsC2sVGSgNV0yIDOoY7qbaqsjiLz-473_UKKeeeg&oe=65F5CDFE
19	Oliwier Tomczuk	1518	https://scontent.xx.fbcdn.net/v/t1.15752-9/428012793_1134404514583966_3069682922561435482_n.png?stp=dst-png_p173x172&_nc_cat=110&ccb=1-7&_nc_sid=510075&_nc_ohc=ohFy-wn5sNkAX_8Lrzc&_nc_ad=z-m&_nc_cid=0&_nc_ht=scontent.xx&oh=03_AdQHsSiZrNlCS7Y9xDjEwbZ0Zn0g-Ria9C7B__LIeHn9wg&oe=65F5C194
25	temp 3	1000	https://cdn.pixabay.com/photo/2013/07/12/12/37/number-146023_640.png
27	temp 5	1000	https://cdn.pixabay.com/photo/2013/07/12/16/22/number-150794_640.png
26	temp 4	1000	https://cdn.pixabay.com/photo/2013/07/12/16/22/number-150793_640.png
23	temp 1	1000	https://cdn.pixabay.com/photo/2016/03/31/17/41/glossy-1293833_640.png
24	temp 2	1000	https://cdn.pixabay.com/photo/2013/07/12/16/22/number-150791_640.png
\.


--
-- Data for Name: soul; Type: TABLE DATA; Schema: public; Owner: grandkahuna43325
--

COPY public.soul (id, owner, name) FROM stdin;
11	12	Siluk
6	7	Jacek
2	2	Odziom
8	9	Filip Gomółka 
19	20	Bartek
17	18	Wieczorek
10	11	Sebastian Gałecki 
21	22	Piotr Gomółka 
5	6	Antoni 
3	3	Nowoszow
4	5	Nigor
1	1	Franek
9	10	Olek Kot
20	21	Klaudia 
18	19	Oliwier Tomczuk
7	8	Żaba
22	23	temp 1
23	24	temp 2
24	25	temp 3
25	26	temp 4
26	27	temp 5
\.


--
-- Name: admin_id_seq; Type: SEQUENCE SET; Schema: public; Owner: grandkahuna43325
--

SELECT pg_catalog.setval('public.admin_id_seq', 4, true);


--
-- Name: logs_id_seq; Type: SEQUENCE SET; Schema: public; Owner: grandkahuna43325
--

SELECT pg_catalog.setval('public.logs_id_seq', 28, true);


--
-- Name: player_id_seq; Type: SEQUENCE SET; Schema: public; Owner: grandkahuna43325
--

SELECT pg_catalog.setval('public.player_id_seq', 27, true);


--
-- Name: soul_id_seq; Type: SEQUENCE SET; Schema: public; Owner: grandkahuna43325
--

SELECT pg_catalog.setval('public.soul_id_seq', 26, true);


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
-- Name: logs logs_pkey; Type: CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.logs
    ADD CONSTRAINT logs_pkey PRIMARY KEY (id);


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
-- Name: logs logs_admin_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.logs
    ADD CONSTRAINT logs_admin_id_fkey FOREIGN KEY (admin_id) REFERENCES public.admin(id);


--
-- Name: soul soul_owner_fkey; Type: FK CONSTRAINT; Schema: public; Owner: grandkahuna43325
--

ALTER TABLE ONLY public.soul
    ADD CONSTRAINT soul_owner_fkey FOREIGN KEY (owner) REFERENCES public.player(id);


--
-- PostgreSQL database dump complete
--

