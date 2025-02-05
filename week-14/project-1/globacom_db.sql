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
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size character(50) NOT NULL,
    data_duration integer NOT NULL,
    data_price integer NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    id integer NOT NULL,
    dept character(50) NOT NULL,
    emp_id integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: departments; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.departments (
    dept_managerid integer NOT NULL,
    dept_no integer NOT NULL,
    dept_name text NOT NULL,
    dept_location text NOT NULL,
    p_no integer NOT NULL
);


ALTER TABLE public.departments OWNER TO postgres;

--
-- Name: employee; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.employee (
    id integer NOT NULL,
    name text NOT NULL,
    age integer NOT NULL,
    address character(50),
    salary real,
    join_date date
);


ALTER TABLE public.employee OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname text NOT NULL,
    pduration character(50),
    project_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

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
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB                                             	2	200
2	1.8GB                                             	14	500
3	3.9GB                                             	30	1000
4	7.5GB                                             	30	1500
5	9.2GB                                             	30	2000
6	10.8GB                                            	30	2500
7	14GB                                              	30	3000
8	18GB                                              	30	4000
9	24GB                                              	30	5000
10	29.9GB                                            	30	8000
11	50GB                                              	30	10000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (id, dept, emp_id) FROM stdin;
\.


--
-- Data for Name: departments; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.departments (dept_managerid, dept_no, dept_name, dept_location, p_no) FROM stdin;
108	1	Administration	Ikeja	44
101	2	Account	Egbeda	11
100	3	Packaging	Ajah	44
120	4	Research	V.I	33
97	5	Account	Magodo	22
122	6	Operations	Mile 2	44
107	7	Packaging	Ketu	55
\.


--
-- Data for Name: employee; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.employee (id, name, age, address, salary, join_date) FROM stdin;
1	Anthony	32	Ebonyi                                            	35000	2003-05-06
2	Alex	24	Rivers                                            	45000	2007-07-24
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
11	A	9 Months                                          	102
22	B	14 Months                                         	97
33	C	16 Months                                         	120
44	D	25 Months                                         	108
55	E	9 Months                                          	107
\.


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
-- Name: employee company_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.employee
    ADD CONSTRAINT company_pkey PRIMARY KEY (id);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (id);


--
-- Name: departments departments_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.departments
    ADD CONSTRAINT departments_pkey PRIMARY KEY (dept_managerid);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (employeeid);


--
-- PostgreSQL database dump complete
--

