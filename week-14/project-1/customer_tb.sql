--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email character(50),
    c_mobile character(50),
    e_id integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, e_id, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com                                 	8034567843                                        	102	5
111	Lilian Jaiya	43	l_jaiye@gmail.com                                 	8038987653                                        	100	3
112	Arthur Musa	50	a_musa@gmail.com                                  	8038456723                                        	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com                                	8034532723                                        	100	2
114	Marylene Mapa	33	m_mapa@gmail.com                                  	8034532723                                        	120	5
115	Oghenero Agor	50	o_agor@gmail.com                                  	8034765433                                        	117	11
116	Adams Bree	33	a_bree@gmail.com                                  	90654765433                                       	102	1
117	Okafor Mathias	45	o_mathias@gmail.com                               	90654542135                                       	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com                               	9063345135                                        	117	11
119	Lawal Tamire	35	l_tamire@gmail.com                                	9066527135                                        	107	5
120	James Job	44	j_job@gmail.com                                   	9066509876                                        	100	8
121	Matthew Jakande	21	m_jakande@gmail.com                               	9010293876                                        	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com                              	9065739876                                        	107	5
\.


--
-- PostgreSQL database dump complete
--

