use leptos::*;
use serde::Deserialize;
use leptos_router::*;
use leptos_meta::*;

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <Router>
                <nav id="topNavWrapper">
                    <div id="topNavContainer">
                        <a href="/">"Home"</a>
                        <a href="/resume">"Resume"</a>
                    </div>
                </nav>
                <main>
                    <Routes>
                        <Route path="/" view=|cx| view! {cx, <h1>"Home"</h1>} />
                        <Route path="/resume" view=|cx| view! {cx, <Resume />}/>
                    </Routes>
                </main>
            </Router>
        }
    })
}

#[component]
fn Resume(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="resumeWrapper">
            <div class="resumeContainer">
                <Script id="jspdf" src="https://cdnjs.cloudflare.com/ajax/libs/jspdf/2.5.1/jspdf.umd.min.js"></Script>
                <script>
                    var doc = undefined;
                    var script = document.querySelector("#jspdf");
                    script.addEventListener("load", function(){
                        doc = new window.jspdf.jsPDF();
                    });
                    console.log(doc);
                </script>
                <section class="resumeBody">
                <div class="main">
                    <section class="resumeHeader">
                        <h1>"Christopher Repka"</h1>
                        <p>"Experienced full-stack developer with a broad and diverse skillset. Self-motivated, collaborative, life-long learner equipped with knowledge and the humility to know where I lack it."</p>
                        <div>
                            "Austin, Texas | "
                            <a href="mailto:repkachristopher@gmail.com">"repkachristopher@gmail.com"</a> " | "
                            <a href="tel:6363585855">"636-358-5855"</a>
                        </div>
                    </section>
                    <h2>"Employment History"</h2>
                    <EmployerComponent
                        employer = Employer{
                            name: String::from("Texas Water Development Board"),
                            positions: vec![
                                Position{
                                    city: String::from("Austin"),
                                    state: String::from("Texas"),
                                    job_title: String::from("Software Developer - Team Lead"),
                                    date_start: String::from("May 2023"),
                                    date_end: String::from("Present"),
                                    accomplishments: vec![
                                        String::from("Helped build a new team of four developers who I train, mentor, collaborate with and learn from daily."),
                                        String::from("Triage with third parties and IT department to secure our cloud, develop new products and services, and ensure compliance with state regulations."),
                                        String::from("Maintain communications with UX, marketing, and project management teams to ensure deadlines and requirements are met for new features and services."),
                                        String::from("Performed a migration of an EKS Cluster and RDS Aurora Database to a new VPC with zero downtime.")
                                    ],
                                },
                                Position{
                                    city: String::from("Austin"),
                                    state: String::from("Texas"),
                                    job_title: String::from("Software Developer"),
                                    date_start: String::from("August 2020"),
                                    date_end: String::from("May 2023"),
                                    accomplishments: vec![
                                        String::from("Create and maintain Kubernetes Clusters, ECS Clusters, EC2s, and Lambda Functions to deploy over twelve web applications and services."),
                                        String::from("Create and maintain CI/CD Pipelines for all services and apps using AWS Codebuild and CodePipeline."),
                                        String::from("Use Git for version control with GitHub and CodeCommit, as well as Elastic Container Repository for versioned container images."),
                                        String::from("Transform server side rendered web applications to statically generated web applications to lower costs by  greater than 90% and reduce cybersecurity threat surfaces. Reduces time and funds spent patching servers and ensuring server uptime by offloading our server footprint to the client."),
                                        String::from("Collaborate with agency partners to gather software requirements, develop software, and garner feedback from various stakeholders."),
                                        String::from("Manage AWS infrastructure as Infrastructure as Code via CDK, Cloudformation, and Terraform."),
                                    ],
                                }
                            ]

                        }
                    />
                    <EmployerComponent
                        employer = Employer{
                            name: String::from("Texas Global"),
                            positions: vec![
                                Position{
                                    city: String::from("Austin"),
                                    state: String::from("Texas"),
                                    job_title: String::from("Web Projects Assistant"),
                                    date_start: String::from("June 2019"),
                                    date_end: String::from("February 2020"),
                                    accomplishments: vec![
                                        String::from("Administered content for two WordPress sites and one Drupal site."),
                                        String::from("Wrote HTML, Javascript, CSS, and PHP to create new pages and features for WordPress and Drupal sites."),
                                        String::from("Performed cross-browser tests using Selenium."),
                                        String::from("Performed database backups and migrations for WordPress sites."),
                                        String::from("Assisted with website migration and rebranding process for 300+ pages of content."),
                                    ],
                                },
                            ],
                        }
                    />
                </div>
                <div class="asideWrapper">
                    <div class="asideContainer">
                        <DegreeListComponent degreez=vec![
                            EdDegree{
                                institution: String::from("University of Texas at Austin"),
                                degree: String::from("Master of Science, Information Studies"),
                                graduation_date: String::from("May, 2020"),
                            },
                            EdDegree{
                                institution: String::from("St. Mary's University"),
                                degree: String::from("Bachelor of Arts, Public History"),
                                graduation_date: String::from("May, 2018"),
                            },
                        ] />
                        <SkillzComponent skillz=vec![
                            String::from("Python"),
                            String::from("Django"),
                            String::from("Flask"),
                            String::from("Javascript"),
                            String::from("React"),
                            String::from("Svelte"),
                            String::from("Go"),
                            String::from("Kubernetes"),
                            String::from("Helm Charts"),
                            String::from("Docker"),
                            String::from("AWS Cloud"),
                            String::from("PostgreSQL"),
                            String::from("Leadership"),
                            String::from("Usability Studies"),
                            String::from("Team Building"),
                            String::from("Collaboration / consulting"),
                            String::from("Documentation"),
                        ] />
                    </div>
                </div>
                </section>
            </div>
        </div>

    }
}

#[component]
fn EmployerComponent(cx: Scope, employer: Employer) -> impl IntoView {
    view! {
        cx,
        <section class="jobDescription">
            <h3 class="jobTitle">{employer.name}</h3>
            {employer.positions.into_iter().map(|p| view! {cx, 
                <div class="positionHeader">
                    <h4 class="jobDateRange">{p.job_title}</h4>
                    <h5 class="jobLocation">{p.date_start} - {p.date_end} | {p.city}, {p.state}</h5>
                </div>
               
                <div class="jobAccomplishments">
                    <ul>
                    {p.accomplishments.into_iter().map(|n| view! {
                        cx,
                        <li>{n}</li>
                    }).collect::<Vec<_>>()}
                    </ul>
                </div>
            }).collect::<Vec<_>>()}
            
        </section>
    }
}

#[component]
fn SkillComponent(cx: Scope, skill_name: String) -> impl IntoView {
    view! {
        cx,
        <li class="skillsListItem">{skill_name}</li>
    }
}

#[component]
fn SkillzComponent(cx: Scope, skillz: Vec<String>) -> impl IntoView {
    view! {
        cx,
        <section class="skillsComponent">
            <h2>"Skills"</h2>
            <ul class="skillsList">
                {skillz.into_iter().map(|n| view! {
                    cx,
                    <SkillComponent skill_name={n} />
                }).collect::<Vec<_>>()}
            </ul>
        </section>
        
    }
}

#[component]
fn DegreeItemComponent(cx: Scope, d: EdDegree) -> impl IntoView {
    view! {
        cx,
            <h4 class="educationalDegreeInstitution">{d.institution}</h4>
            <ul><li>{d.degree} | {d.graduation_date}</li></ul>
    }
}

#[component]
fn DegreeListComponent(cx: Scope, degreez: Vec<EdDegree>) -> impl IntoView {
    view! {
        cx,
        <section class="degreeListComponent">
            <h2>"Education"</h2>
            <div class="degreeList">
                {degreez.into_iter().map(|n| view! {
                    cx,
                    <DegreeItemComponent d={n} />
                }).collect::<Vec<_>>()}
            </div>
        </section>
        
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Position {
    city: String,
    state: String,
    job_title: String,
    date_start: String,
    date_end: String,
    accomplishments: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Employer {
    name: String,
    positions: Vec<Position>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EdDegree {
    institution: String,
    degree: String,
    graduation_date: String,
}
