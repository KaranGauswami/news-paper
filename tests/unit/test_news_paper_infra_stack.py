import aws_cdk as core
import aws_cdk.assertions as assertions

from news_paper_infra.news_paper_infra_stack import NewsPaperInfraStack

# example tests. To run these tests, uncomment this file along with the example
# resource in news_paper_infra/news_paper_infra_stack.py
def test_sqs_queue_created():
    app = core.App()
    stack = NewsPaperInfraStack(app, "news-paper-infra")
    template = assertions.Template.from_stack(stack)

#     template.has_resource_properties("AWS::SQS::Queue", {
#         "VisibilityTimeout": 300
#     })
