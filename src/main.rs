use components::resume::{DegreeListComponent, EmployerComponent, SkillzComponent};

use components::resume::{EdDegree, Employer, Position};
use leptos::*;
use leptos_meta::*;
mod components;

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,

                  <Resume />

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
                                Position {
                                    city: String::from("Austin"),
                                    state: String::from("Texas"),
                                    job_title: String::from("Manager - Application Development"),
                                    date_start: String::from("May 2023"),
                                    date_end: String::from("Present"),
                                    accomplishments: vec![
                                        String::from("Helped build a new team of four developers who I train, mentor, collaborate with and learn from daily."),
                                        String::from("Triage with third parties and IT department to secure our cloud, develop new products and services, and ensure compliance with state regulations."),
                                        String::from("Maintain communications with UX, marketing, and project management teams to ensure deadlines and requirements are met for new projects, features, and services."),
                                        String::from("Help establish strategic plans and communicate value and limitations of software and web application projects."),
                                        String::from("Performed the migration of an AWS Kubernetes Cluster and PostgreSQL Database to a new Virtual Private Cloud with zero application downtime.")
                                    ],
                                },
                                Position {
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
                            ],
                        }
                    />
                    <EmployerComponent
                    employer = Employer{
                        name: String::from("The University of Texas at Austin"),
                        positions: vec![
                            Position{
                                city: String::from("Austin"),
                                state: String::from("Texas"),
                                job_title: String::from("Teaching Assistant"),
                                date_start: String::from("August 2018"),
                                date_end: String::from("May 2020"),
                                accomplishments: vec![
                                    String::from("Created video resources such as tutorials and introductions to online course materials and concepts"),
                                    String::from("Assisted the production and maintenance of course materials for online courses using Canvas"),
                                    String::from("Assisted the formulation of innovative solutions to distance learning needs and requirements"),
                                    String::from("Collaborated with UT faculty and Lab Staff coworkers to solve problems"),
                                    String::from("Maintained order of the IT Lab, including over 70 computers running a mixture of Mac OS and Windows"),
                                    String::from("Provided technology assistance services to faculty and students on the use of IT Lab tech and software such as the Double Telepresence Robot, sound recording rooms, Camtasia, Zoom, Panopto, and the Adobe Creative Suite"),
                                    String::from("Managed work orders and resource reservations through Spiceworks, Outlook, and the Google Suite"),
                                ],
                            },
                        ],
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
