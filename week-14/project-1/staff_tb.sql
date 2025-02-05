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
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    employeeid integer NOT NULL,
    employee_name text NOT NULL,
    department_no integer NOT NULL,
    employee_salary real,
    age integer NOT NULL,
    phone_no character varying(50)
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (employeeid, employee_name, department_no, employee_salary, age, phone_no) FROM stdin;
101	Alade Joy	2	250000	33	8098765987
100	Mustapha Ali	3	175000	32	8064323458
102	Mankinde Mary	2	450000	55	9023456776
120	Adeleke Jane	4	200000	38	7065432890
122	Osahon Mark	6	320000	44	8065432789
117	Suleman Ajayi	3	800000	50	7098765678
104	Kuti Lawal	1	750000	35	9145345672
97	Dankade Aminat	5	550000	40	9087654329
108	Joisiah Joshua	1	120000	30	8098765432
107	Alokwe Martin	7	380000	48	7009876340
\.


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (employeeid);


--
-- PostgreSQL database dump complete
--

