from aws_cdk import (
    # Duration,
    Stack,
    # aws_sqs as sqs,
    aws_ec2 as ec2,
    aws_ec2 as aws_ec2,
    aws_ecs as ecs,
    aws_ecs_patterns as ecs_patterns,
    aws_applicationautoscaling as appscaling,
    aws_ecr_assets,
)
from constructs import Construct
import os


class NewsPaperInfraStack(Stack):

    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        # The code that defines your stack goes here
        # Reference the default VPC
        default_vpc = ec2.Vpc.from_lookup(self, "DefaultVpc", is_default=True)
        default_sg = ec2.SecurityGroup.from_lookup_by_name(
            self, "DefaultSecurityGroup", vpc=default_vpc, security_group_name="default"
        )

        cluster = ecs.Cluster(
            self,
            "NewsPaperCluster",
            enable_fargate_capacity_providers=True,
            vpc=default_vpc,
        )
        cluster.add_default_capacity_provider_strategy(
            default_capacity_provider_strategy=[
                ecs.CapacityProviderStrategy(
                    capacity_provider="FARGATE_SPOT", base=10, weight=50
                )
            ]
        )

        new_defination = ecs.FargateTaskDefinition(
            self,
            "NewsPaperTaskDefinition2",
            cpu=1024,
            memory_limit_mib=4096,
            runtime_platform=ecs.RuntimePlatform(
                operating_system_family=ecs.OperatingSystemFamily.LINUX,
                cpu_architecture=ecs.CpuArchitecture.ARM64,
            ),
        )
        asset = aws_ecr_assets.DockerImageAsset(self, "MyBuildImage", directory="code")
        new_defination.add_container(
            "NewsPaperContainer2",
            image=ecs.ContainerImage.from_docker_image_asset(asset),
            environment={
                "BOT_TOKEN": os.getenv("BOT_TOKEN"),
                "CHAT_IDS": os.getenv("CHAT_IDS"),
            },
            logging=ecs.LogDriver.aws_logs(stream_prefix="NewsPaperContainerLogs"),
        )
        ecs_patterns.ScheduledFargateTask(
            self,
            "ScheduledFargateTask2",
            cluster=cluster,
            scheduled_fargate_task_definition_options=ecs_patterns.ScheduledFargateTaskDefinitionOptions(
                task_definition=new_defination
            ),
            desired_task_count=1,
            schedule=appscaling.Schedule.cron(minute="30", hour="1"),
            # tags=[Tag(key="my-tag", value="my-tag-value")],
            subnet_selection=ec2.SubnetSelection(subnet_type=ec2.SubnetType.PUBLIC),
            security_groups=[default_sg],
        )
