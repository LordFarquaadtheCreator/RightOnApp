fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

// current focus wordpress only
// idea: "slow mode" where it goes through each step and asks you to verify
slint::slint! {
    export component MainWindow inherits Window {
        width: 800*1px;
        height: 600*1px;

        // main window everything stacks vertically
        VerticalLayout{
            height: 600*1px;

            // horizontal box for padding/logo
            HorizontalLayout {
                height: 100*1px;
                Rectangle { 
                    background: #D8E2DC; 
                    Text { text: "Wordpress Uploader"; font-size: 30*1px; color: black; }
                }
            }

            HorizontalLayout{
                height: 400*1px;
                Rectangle { background: #FFFFFF; }
            }
            // vertical box for uploading email text

            // vertical box for uploading media

            // horizontal box on the bottom for submitting & triggering wordpress api call
            HorizontalLayout{
                height: 100*1px;
                Rectangle { background: #FFCAD4; }
            }
        }
        
    }
}