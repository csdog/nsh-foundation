use components::rsx;
use nsh_core::prelude::IntoElement;

pub fn example() -> impl IntoElement {
    rsx! {
        <Card>
            <CardHeader>
                <CardTitle>Project settings</CardTitle>
                <CardDescription>Manage this project.</CardDescription>
            </CardHeader>
            <CardContent>
                <Text>Visibility: Private</Text>
            </CardContent>
            <CardFooter>
                <Button href="https://nsh.dev/docs/"><Text>Native Shell docs</Text></Button>
            </CardFooter>
        </Card>
    }
}
