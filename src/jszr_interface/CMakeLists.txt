cmake_minimum_required(VERSION 3.8)
project(jszr_robots_dog_msg
    VERSION 0.0.1
)

set(CMAKE_CXX_STANDARD 17)

find_package(ament_cmake REQUIRED)
find_package(std_msgs REQUIRED)
find_package(rosidl_default_generators REQUIRED)


set(msg_files
  "msg/MotorCmd.msg"
  "msg/MotorState.msg"
  "msg/BmsCmd.msg"
  "msg/BmsState.msg"
  "msg/Cartesian.msg"
  "msg/IMU.msg"
  "msg/LED.msg"
  "msg/LowCmd.msg"
  "msg/LowState.msg"
  "msg/HighCmd.msg"
  "msg/HighState.msg"
  "msg/CustomMsg.msg"
  "msg/CustomPoint.msg"
  "msg/Pose6D.msg"
)

# 使用rosidl生成器生成自定义消息
rosidl_generate_interfaces(${PROJECT_NAME}
  ${msg_files}
  DEPENDENCIES std_msgs  # 如果消息中包含标准类型
)

# 安装其他项目的需求
ament_export_dependencies(rosidl_default_runtime)
install(
  DIRECTORY msg
  DESTINATION share/${PROJECT_NAME}
)

ament_package()

# enabel deb package
set(CPACK_PACKAGING_INSTALL_PREFIX "/opt/ros/$ENV{ROS_DISTRO}")
set(CPACK_PACKAGE_NAME ${PROJECT_NAME})
set(CPACK_PACKAGE_FILE_NAME ${CPACK_PACKAGE_NAME}_${PROJECT_VERSION}_${CMAKE_HOST_SYSTEM_PROCESSOR}_$ENV{ROS_DISTRO}_${CMAKE_SYSTEM_NAME})
set(CPACK_GENERATOR "DEB")
set(CPACK_DEBIAN_PACKAGE_MAINTAINER "kanonwy")

include(CPack)

