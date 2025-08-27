//#include <pinocchio/fwd.hpp>
#include "pinocchio/algorithm/kinematics.hpp"
#include "pinocchio/algorithm/joint-configuration.hpp"
#include "pinocchio/parsers/urdf.hpp"
#include <iostream>

extern "C" {
    // Function to compute the end-effector pose (both translation and rotation)
    void compute_end_effector_pose() {
        // Load the model from the URDF file
        const std::string urdf_filename = std::string("models/baxter_simple.urdf");//std::string(urdf_path);
       
        // Load the urdf model
        pinocchio::Model model;
        pinocchio::urdf::buildModel(urdf_filename, model);
        std::cout << "model name: " << model.name << std::endl;
       
        // Create data required by the algorithms
        pinocchio::Data data(model);

        Eigen::VectorXd q = pinocchio::neutral(model);

        // Compute Forward Kinematics
        pinocchio::forwardKinematics(model, data, q);

        // Get the end-effector frame (change this to your end-effector's frame name)
        const int frame_id = model.getFrameId("ee_link");
        const pinocchio::SE3& end_effector_pose = data.oMf[frame_id];

        // Output the translation and rotation of the end-effector
        std::cout << "End-effector pose:" << std::endl;
        std::cout << "Translation: " << end_effector_pose.translation().transpose() << std::endl;
        std::cout << "Rotation matrix: " << std::endl << end_effector_pose.rotation() << std::endl;
    }
}

